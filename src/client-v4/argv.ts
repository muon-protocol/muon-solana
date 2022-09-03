import yargs from 'yargs/yargs';

type MuonCommandHandlers = {
    initAdmin?: () => Promise<void>
    getAdminInfo?: () => Promise<void>
    transferAdmin?: (argv:any) => Promise<void>
    addGroup?: (argv:any) => Promise<void>
    listGroup?: () => Promise<void>
    verifyTest?: () => Promise<void>
    estimateLamports?: (argv:any) => Promise<void>
}

export async function handleArgs (handlers: MuonCommandHandlers) {
    await yargs(process.argv.slice(2))
        .command("admin-init",
            "initialize admin account.",
            {},
            handlers.initAdmin
        )
        .command("admin-get",
            "retrieve admin account info.",
            {},
            handlers.getAdminInfo
        )
        .command("admin-transfer <newAdmin>",
            "transfer admin to new account.",
            {},
            handlers.transferAdmin
        )
        .command("group-add <ethAddress> <pubKeyX> <pubkeyYParity>",
            "add new verification group info.",
            {
                ethAddress: {type: "string"},
                pubKeyX: {type: "string"},
                pubkeyYParity: {type: "string"},
            },
            handlers.addGroup
        )
        .command("group-list",
            "list all groups already added.",
            {},
            handlers.listGroup
        )
        .command("verify-test",
            "call muon and verify its signature.",
            {},
            handlers.verifyTest
        )
        .command("estimate-lamports <numBytes>",
            "estimate lamports to make account rent exempt.",
            {},
            handlers.estimateLamports
        )
        .demandCommand()
        .help()
        .argv;
}
