//? Libaries
import { Context } from "https://deno.land/x/oak@v12.1.0/mod.ts";

export function test(ctx: Context) {
  ctx.response.body = {
    message: "ok",
  };
}
