import { Router } from "express";
import signInController from "../controllers/user-controllers/controller.sign-in";
import UserAuth from "../controllers/user-controllers/controller.sign-in";

const router = Router();

// auth-routes
router.post("/sign-in", UserAuth.sign_in);

export default router;
