import { Account, AuthOptions, ISODateString } from "next-auth";
import GoogleProvider from "next-auth/providers/google";
import axios from "axios";
import { JWT } from "next-auth/jwt";
import API from "@/src/backend/routes/api_routes";
import { ENV } from "@/config/config.env";

export interface UserType {
    id?: string | null;
    name?: string | null;
    email?: string | null;
    image?: string | null;
    provider?: string | null;
    token?: string | null;
}

export interface CustomSession {
    user?: UserType;
    expires: ISODateString;
}

export const authOption: AuthOptions = {
    pages: {
        signIn: "/",
    },
    callbacks: {
        async signIn({ user, account }: { user: UserType; account: Account | null }) {
            try {
                if (account?.provider === "google") {
                    const response = await axios.post(`${API.SIGNIN_URL}`, {
                        user,
                        account,
                    });

                    const result = response.data;

                    if (result?.success) {
                        user.id = result.data.user.id.toString();
                        user.token = result.data.token;
                        return true;
                    }
                }

                if (account?.provider === "email-otp") {
                    return !!user;
                }

                return false;
            } catch (err) {
                console.error(err);
                return false;
            }
        },
        async jwt({ token, user }) {
            if (user) {
                token.user = user as UserType;
            }
            return token;
        },
        async session({ session, token }: { session: CustomSession; token: JWT }) {
            session.user = token.user as UserType;
            return session;
        },
    },
    providers: [
        GoogleProvider({
            clientId: ENV.GOOGLE_CLIENT_ID,
            clientSecret: ENV.GOOGLE_CLIENT_SECRET,
            authorization: {
                params: {
                    prompt: "consent",
                    access_type: "offline",
                    response_type: "code",
                },
            },
        }),
    ],
};
