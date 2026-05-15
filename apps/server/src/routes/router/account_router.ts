import { Router } from "express";
import UserAccount from "../../controllers/user-controllers/controller.get_balance";

const account_router = Router();

account_router.get("/balance", UserAccount.get_balance);

export default account_router;
