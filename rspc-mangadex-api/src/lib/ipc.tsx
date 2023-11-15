import type { Procedures } from "./bindings";
import { createClient } from "@rspc/client";
import { createReactQueryHooks } from "@rspc/react";
import { TauriTransport } from "@rspc/tauri";
import { QueryClient } from "@tanstack/react-query";
import React from "react";

export const client = createClient<Procedures>({
    transport: new TauriTransport()
});

export const rspc = createReactQueryHooks<Procedures>();

export const queryClient = new QueryClient({
    defaultOptions: {
        queries : {
            "staleTime": 1000 * 60 * 30
        },
    }
});

function Children({ children }: React.PropsWithChildren) {
    return (
        <>
            {children}
        </>
    )
}

export default function IPCProvider({ children }: React.PropsWithChildren){
    return (
        <rspc.Provider client={client} queryClient={queryClient}>
            <Children>
                {children}
            </Children>
        </rspc.Provider>
    )
}