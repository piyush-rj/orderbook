import { Router } from "express";
import UserAuth from "../../controllers/auth-controllers/controller.sign-in";

const auth_router = Router();

auth_router.post("/sign-in", UserAuth.sign_in);

export default auth_router;
