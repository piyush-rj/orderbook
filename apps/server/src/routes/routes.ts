import { Router } from "express";
import auth_router from "./router/auth_router";
import user_router from "./router/account_router";

const router = Router();

router.use("/", auth_router);
router.use("/account", user_router);

export default router;
