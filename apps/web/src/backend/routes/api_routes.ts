import { ENV } from "@/config/config.env";

export default class API {
    static BASE_URL = ENV.NEXT_PUBLIC_BACKEND_URL;
    static SIGNIN_URL = `${this.BASE_URL}/sign-in`;
}
