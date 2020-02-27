import { assert, test, testPerm } from "./test_util.ts";

const openErrorStackPattern = new RegExp(
  `^.*
    at unwrapResponse \\(.*dispatch_json\\.ts:.*\\)
    at Object.sendAsync \\(.*dispatch_json\\.ts:.*\\)
    at async Object\\.open \\(.*files\\.ts:.*\\).*$`,
  "ms"
);

testPerm({ read: true }, async function sendAsyncStackTrace(): Promise<void> {
  await Deno.open("nonexistent.txt")
    .then(() => {
      assert.unreachable();
    })
    .catch((error: Error): void => {
      assert.match(error.stack!, openErrorStackPattern);
    });
});

test(async function malformedJsonControlBuffer(): Promise<void> {
  // @ts-ignore
  const opId = Deno.core.ops()["op_open"];
  // @ts-ignore
  const res = Deno.core.send(opId, new Uint8Array([1, 2, 3, 4, 5]));
  const resText = new TextDecoder().decode(res);
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  const resJson = JSON.parse(resText) as any;
  assert(!resJson.ok);
  assert(resJson.err);
});
