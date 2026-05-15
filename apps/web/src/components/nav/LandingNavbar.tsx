"use client";
import { Button } from "@/src/components/ui/button";
import { useUserSessionStore } from "@/src/store/useUserSessionStore";
import { signIn } from "next-auth/react";
import Image from "next/image";
import { JSX } from "react";

export default function LandingNavbar(): JSX.Element {
    const session = useUserSessionStore((s) => s.session);

    function handleSignIn() {
        console.log("clicked");
        signIn("google", { callbackUrl: "/" });
    }

    return (
        <div className="h-16 w-full max-w-6xl mx-auto flex justify-between items-center fixed top-0 left-1/2 -translate-x-1/2">
            <div>Orderbook</div>
            <div className="">
                {session && session.user?.image ? (
                    <div className="flex gap-1 items-center">
                        <div className="">{session.user.name}</div>
                        <div className="h-8 w-8 rounded-full overflow-hidden">
                            <Image
                                src={session.user?.image}
                                alt=""
                                className="object-cover"
                                fill
                                unoptimized
                            />
                        </div>
                    </div>
                ) : (
                    <Button
                        onClick={handleSignIn}
                        className="bg-white text-neutral-800 rounded-sm cursor-pointer active:scale-[0.99] transition-all transform duration-200"
                    >
                        Sign In
                    </Button>
                )}
            </div>
        </div>
    );
}
