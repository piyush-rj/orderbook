import z from "zod";

const env_schema = z.object({
    NEXT_PUBLIC_BACKEND_URL: z.url(),
    GOOGLE_CLIENT_ID: z.string().min(1),
    GOOGLE_CLIENT_SECRET: z.string().min(1),
});

const parsed = env_schema.safeParse({
    NEXT_PUBLIC_BACKEND_URL: process.env.NEXT_PUBLIC_BACKEND_URL,
    GOOGLE_CLIENT_ID: process.env.GOOGLE_CLIENT_ID,
    GOOGLE_CLIENT_SECRET: process.env.GOOGLE_CLIENT_SECRET,
});

if (!parsed.success) {
    console.error("Invalid environment variables:", z.treeifyError(parsed.error));
    throw new Error("Invalid environment variables");
}

export const ENV: z.infer<typeof env_schema> = parsed.data;
