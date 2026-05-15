import type { Request, Response } from "express";
import z from "zod";
import ResponseWriter from "../../class/response_writer";
import { prisma } from "database";
import type { BalanceResponse } from "@orderbook/types";

export default class UserAccount {
    static account_balance_schema = z.object({});

    static async get_balance(req: Request, res: Response) {
        const user = req.user;
        if (!user || !user.id) {
            ResponseWriter.unauthorized(res);
            return;
        }

        try {
            const user_balance = await prisma.balance.findMany({
                where: {
                    id: user.id,
                },
                include: {
                    asset: true,
                },
            });

            const data = user_balance.map((b) => ({
                asset: b.asset.symbol,
                available: b.available,
                locked: b.locked,
            }));

            ResponseWriter.success(res, data, "Fetched balanced succesfully");
        } catch (error) {
            console.error("get_balance err: ", error);
            ResponseWriter.server_error(res);
        }
    }
}
