#! /usr/bin/env sh
/*
exec deno test --unstable --allow-net --allow-read=./ --allow-run "$0" "$@"
*/

// can't run on linux, https://qastack.cn/unix/63979/shebang-line-with-usr-bin-env-command-argument-fails-on-linux
/// #!/usr/bin/env deno test --unstable --allow-net --allow-read=./ --allow-run

import { Command } from "https://deno.land/x/cliffy@v0.19.2/command/mod.ts";
import * as Colors from "https://deno.land/std@0.100.0/fmt/colors.ts";
import {
  assert,
  assertArrayIncludes,
  assertEquals,
  assertExists,
  assertMatch,
  assertNotEquals,
  assertNotMatch,
  assertStrictEquals,
  assertStringIncludes,
} from "https://deno.land/std@0.100.0/testing/asserts.ts";

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
  .name("md5t")
  .version("0.0.0")
  .description("md5 api tester")
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

// https://deno.land/manual/getting_started/first_steps
const api = `http://${options.host}:${options.port}/`;

// curl -v 0.0.0.0:8000/
Deno.test("GET /", async () => {
  let res = await fetch(api);
  assertEquals(res.status, 200);
  let body = await res.text();
  assertStringIncludes(body, "Hello world!");
});

// curl -v 0.0.0.0:8000/500
Deno.test("GET /500", async () => {
  let res = await fetch(api + "500");
  assertEquals(res.status, 404);
  let body = await res.text();
  assertEquals(body, "");
});

// curl -v 0.0.0.0:8000/assets/
Deno.test("GET /assets/", async () => {
  let res = await fetch(api + "assets");
  assertEquals(res.status, 404);
  let body = await res.text();
  assertEquals(body, "");
});

// curl -v 0.0.0.0:8000/assets/../mod.js
Deno.test("GET /assets/../mod.js", async () => {
  let res = await fetch(api + "assets/../mod.js");
  assertEquals(res.status, 404);
  let body = await res.text();
  assertEquals(body, "");
});

// curl -v 0.0.0.0:8000/assets//ls.txt
Deno.test("GET /assets/ls.txt", async () => {
  let res = await fetch(api + "assets/ls.txt");
  assertEquals(res.status, 200);
  let body = await res.text();
  let file = Deno.readTextFileSync("assets/ls.txt");
  assertEquals(body, file);
});

// curl -v 0.0.0.0:8000/assets//ls..txt
Deno.test("GET /assets/ls..txt", async () => {
  let res = await fetch(api + "assets/ls..txt");
  assertEquals(res.status, 200);
  let body = await res.text();
  let file = Deno.readTextFileSync("assets/ls..txt");
  assertEquals(body, file);
});

// curl -v 0.0.0.0:8000/assets/.gitkeep
Deno.test("GET /assets/.gitkeep", async () => {
  let res = await fetch(api + "assets/.gitkeep");
  assertEquals(res.status, 404);
  let body = await res.text();
  assertEquals(body, "");
});

// curl -v 0.0.0.0:8000/api/md5/yes -v
Deno.test("GET /api/md5/yes", async () => {
  let res = await fetch(api + "api/md5/yes");
  assertEquals(res.status, 200);
  let body = await res.json();
  assertEquals(body.code, 200);
  assertEquals(body.data.input, "yes");
  assertEquals(body.data.md5, "a6105c0a611b41b08f1209506350279e");
  assert(new Date(body.data.iso));
});

// curl -v 0.0.0.0:8000/api/md5 -d 'yes' -v
Deno.test("POST /api/md5", async () => {
  let res = await fetch(api + "api/md5", { method: "POST", body: "yes" });
  assertEquals(res.status, 200);
  let body = await res.json();
  assertEquals(body.code, 200);
  assertEquals(body.data.input, "yes");
  assertEquals(body.data.md5, "a6105c0a611b41b08f1209506350279e");
  assert(new Date(body.data.iso));
});
