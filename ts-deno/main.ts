import { become_supporter } from "../rust_wasm/pkg-deno/rust_wasm.js";

// Learn more at https://docs.deno.com/runtime/manual/examples/module_metadata#concepts
if (import.meta.main) {
  const supporter = become_supporter("Alan", 2015);

  // Alan has been a supporter of Vissel Kobe since 2015!
  console.log(supporter.say());
}
