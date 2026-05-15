import z from "zod";
import { SIDE, ORDER_TYPE, ORDER_STATUS } from "./enums.ts";

export const decimal_string = z.string().regex(/^\d+(\.\d+)?$/);

export const NewOrderRequest = z.object({
    clientOrderId: z.string().min(1).max(64),
    market: z.string(),
    side: z.enum([SIDE.BUY, SIDE.SELL]),
    type: z.enum([ORDER_TYPE.LIMIT, ORDER_TYPE.MARKET]),
    price: decimal_string,
    quantity: decimal_string,
});

export const NewOrderResponse = z.object({
    orderId: z.string(),
    clientOrderId: z.string(),
    status: ORDER_STATUS,
    filledQuantity: decimal_string,
});

export const CancelOrderRequest = z.object({
    clientCancelId: z.string().min(1).max(64).optional(),
});

export const BalanceResponse = z.object({
    asset: z.string(),
    available: decimal_string,
    locked: decimal_string,
});

export const OrderRowResponse = z.object({
    orderId: z.string(),
    clientOrderId: z.string(),
    market: z.string(),
    side: z.enum([SIDE.BUY, SIDE.SELL]),
    type: z.enum([ORDER_TYPE.LIMIT, ORDER_TYPE.MARKET]),
    price: decimal_string,
    quantity: decimal_string,
    filledQuantity: decimal_string,
    status: ORDER_STATUS,
    createdAt: z.string().datetime(),
});

export type NewOrderRequest = z.infer<typeof NewOrderRequest>;
export type NewOrderResponse = z.infer<typeof NewOrderResponse>;
export type CancelOrderRequest = z.infer<typeof CancelOrderRequest>;
export type BalanceResponse = z.infer<typeof BalanceResponse>;
export type OrderRowResponse = z.infer<typeof OrderRowResponse>;
