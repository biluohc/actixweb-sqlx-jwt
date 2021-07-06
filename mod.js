#! /usr/bin/env sh
/*
exec deno run --unstable --allow-net --allow-read=./ --allow-run "$0" "$@"
*/

// can't run on linux, https://qastack.cn/unix/63979/shebang-line-with-usr-bin-env-command-argument-fails-on-linux
/// #!/usr/bin/env deno run --unstable --allow-net --allow-read=./ --allow-run

import { Command } from "https://deno.land/x/cliffy@v0.19.2/command/mod.ts";
import * as Colors from "https://deno.land/std@0.100.0/fmt/colors.ts";
import {
  Application,
  isErrorStatus,
  Router,
  send,
  Status,
  STATUS_TEXT,
} from "https://deno.land/x/oak@v7.7.0/mod.ts";

let debug = false;
console.debug = function (...msg) {
  if (debug) {
    console.log(Colors.blue("[DEBU]"), now(), ...msg);
  }
};
console.info = function (...msg) {
  console.log(Colors.green("[INFO]"), now(), ...msg);
};
console.warn = function (...msg) {
  console.log(Colors.yellow("[WARN]"), now(), ...msg);
};
console.error = function (...msg) {
  console.log(Colors.red("[ERRO]"), now(), ...msg);
};

const { options, args } = await new Command()
  .name("md5s")
  .version("0.0.0")
  .description("md5 api server")
  .helpOption("-H, --help", "Print help info.", { global: true })
  .option("-d, --debug", "Print debug info.", { global: true })
  .option("-p, --port <port:integer>", "the port number.", { default: 8000 })
  .option("-h, --host [hostname]", "the host name.", { default: "127.0.0.1" })
  .arguments("[input...:string]", "input strings.")
  .parse(Deno.args);
options["args"] = args[0] ?? [];
debug = options.debug;

console.debug("options: ", options);

function now() {
  return (new Date()).toISOString("en-Us");
}

const encoder = new TextEncoder();
const decoder = new TextDecoder();
const cmd = Deno.build.os == "darwin" ? "md5" : "md5sum";

async function md5(arg) {
  const p = Deno.run({
    cmd: ["bash"],
    stdout: "piped",
    stdin: "piped",
  });

  const command = `echo -n '${arg}' | ${cmd}`;
  console.debug("cmd:", command);
  await p.stdin.write(encoder.encode(command));
  p.stdin.close();

  const output = await p.output();
  p.close();
  return decoder.decode(output).trim().split("-")[0].trim();
}

async function md5json(input) {
  console.debug("input:", input);
  const res = await md5(input);
  return new apiResult({
    input,
    md5: res,
    iso: now(),
  }).json();
}

if (options.args.length > 0) {
  for (const arg of options.args) {
    console.info(`'${arg}':`, await md5(arg));
  }
  Deno.exit(0);
}

const app = new Application();

// Timing && Logger
app.use(async (ctx, next) => {
  const start = Date.now();

  await next();
  const rt = Date.now() - start;
  ctx.response.headers.set("X-Response-Time", `${rt}ms`);

  console.info(
    `${ctx.request.ip} ${Colors.bold(ctx.request.method)
    } ${ctx.request.url} ${ctx._api.code}/${Colors.bold("" + ctx.response.status)
    } '${ctx._api.msg}' - ${rt}ms`,
  );
});

function apiResult(data = null) {
  this.code = 200;
  this.msg = "";
  this.data = data;

  this.json = function () {
    return JSON.stringify(this);
  };
}

app.use(async (ctx, next) => {
  const f = function (ctx) {
    let api = new apiResult();
    api.code = ctx.response.status;
    api.msg = STATUS_TEXT.get(api.code);
    // console.info(ctx.response.body)
    if (
      isErrorStatus(api.code) && !ctx.response.body &&
      ctx.request.url.pathname.startsWith("/api")
    ) {
      ctx.response.body = api.json();
      ctx.response.type = "application/json";
    }
    ctx._api = api;
  };

  try {
    await next();
  } catch (err) {
    console.error(
      `${ctx.request.ip} ${Colors.bold(ctx.request.method)
      } ${ctx.request.url} ${ctx.response.status.toString()}: ${err}`,
    );
  } finally {
    f(ctx);
  }
});

const assets = "/assets";
const assets_rpath = Deno.cwd() + assets;
const router = new Router();
router
  // curl -v 0.0.0.0:8000/ -v
  .get("/", (ctx) => {
    ctx.response.body = "Hello world!";
  })
  // curl -v 0.0.0.0:8000/500 -v
  .get("/500", (ctx) => {
    throw "500 test";
  })
  .get(
    `${assets}/(.*)`,
    async (ctx) => {
      let path = ctx.request.url.pathname.replaceAll(
        /\/+/g,
        "/",
      ).slice(assets.length);

      console.debug("path: ", path);

      await send(ctx, path, { root: assets_rpath });
    },
  )
  // curl -v 0.0.0.0:8000/api/md5 -d 'yes' -v
  .post("/api/md5", async (ctx) => {
    const input = await ctx.request.body({ type: "text" })?.value ?? "";
    ctx.response.body = await md5json(input);
    ctx.response.type = "application/json";
  })
  // curl -v 0.0.0.0:8000/api/md5/yes -v
  .get("/api/md5/:input", async (ctx) => {
    const input = ctx?.params?.input ?? "";
    ctx.response.body = await md5json(input);
    ctx.response.type = "application/json";
  });

app.use(router.routes());
app.use(router.allowedMethods());

app.addEventListener("listen", ({ hostname, port, secure }) => {
  console.warn(
    `Listening on: ${secure ? "https://" : "http://"}${hostname ??
    "localhost"}:${port}`,
  );
});
await app.listen({ port: options.port, hostname: options.host });
