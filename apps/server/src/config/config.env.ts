import z from "zod";

const env_schema = z.object({
    SERVER_JWT_SECRET: z.string(),
    SERVER_PORT: z.coerce.number(),
    DATABASE_URL: z.url(),
    SERVER_DEV_TYPE: z.enum(["development", "production"]).default("development"),
});

export let ENV: z.infer<typeof env_schema>;

export function parse_env() {
    try {
        ENV = env_schema.parse(process.env);
    } catch (err) {
        console.error("Failed to parse the env config: ", err);
        process.exit(1);
    }
}
