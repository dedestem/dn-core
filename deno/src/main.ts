//? Libraries
import { Application, Context } from "https://deno.land/x/oak@v12.1.0/mod.ts";
import { config } from "https://deno.land/x/dotenv@v3.2.2/mod.ts";
import { oakCors } from "https://deno.land/x/cors@v1.2.2/mod.ts";

//? Routes
//? ./
import { test } from "./routes/test.ts";

//? Objects
const app = new Application();
const environment = config();
const port = Number(environment.API_PORT);

//? CORS
app.use(oakCors({
    origin: ["https://account.davidnet.net", "https://davidnet.net"],
    methods: ["POST", "OPTIONS"],
    allowedHeaders: ["Content-Type"],
    credentials: true,
}));

//? Routes
app.use(async (ctx: Context) => {
    //? General
    if (
        ctx.request.method === "GET" &&
        ctx.request.url.pathname === "/"
    ) {
        ctx.response.body = { message: "Access denied!" };
        return;
    }

    if (
        ctx.request.method === "GET" &&
        ctx.request.url.pathname === "/test"
    ) {
        test(ctx);
    }
});

// Start the server
console.log(`Server running at http://localhost:${port}`);
await app.listen({ port: port });
