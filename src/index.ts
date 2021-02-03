import { Context } from "../lib/Cargo.toml";

const ctx = Context.new();
ctx.write("test.txt", "hellow wasm!");
ctx.git_add("test.txt");
ctx.git_commit("commit from wasm");
const hash = ctx.read_head();
console.log(ctx.cat_file_p(hash));
