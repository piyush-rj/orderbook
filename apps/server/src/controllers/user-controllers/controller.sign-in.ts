import type { Request, Response } from "express";
import { prisma } from "database";
import ResponseWriter from "../../class/response_writer";
import z from "zod";
import type { JWTPayload } from "../../services/jwt/services.jwt";
import JwtServices from "../../services/jwt/services.jwt";

export default class UserAuth {
    static user_auth_schema = z.object({
        name: z.string(),
        email: z.email(),
        image: z.string(),
    });

    static async sign_in(req: Request, res: Response) {
        const { data, success } = this.user_auth_schema.safeParse(req.body);
        if (!success) {
            ResponseWriter.invalid_data(res);
            return;
        }

        try {
            const auth_user = await prisma.user.upsert({
                where: { email: data.email },
                update: {
                    name: data.name,
                    email: data.email,
                    image: data.image,
                },
                create: data,
            });

            const jwt_payload: JWTPayload = {
                id: auth_user.id,
                name: auth_user.name,
                email: auth_user.email,
                image: auth_user.image,
            };

            const token = JwtServices.assign_token(jwt_payload);

            ResponseWriter.success(res, { user: auth_user, token: token }, "Sign in successfull");
        } catch (err) {
            console.error("signin controller failed: ", err);
            ResponseWriter.server_error(res);
        }
    }
}
