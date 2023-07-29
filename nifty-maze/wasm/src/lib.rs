// Code generated by the multiversx-sc multi-contract system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Endpoints:                           46
// Async Callback (empty):               1
// Total number of exported functions:  48

#![no_std]
#![feature(lang_items)]

multiversx_sc_wasm_adapter::allocator!();
multiversx_sc_wasm_adapter::panic_handler!();

multiversx_sc_wasm_adapter::endpoints! {
    nifty_maze
    (
        ping
        addMove
        mvpClaim
        getRewardTokens
        getPaymentToken
        getPaymentAmount
        getPlayerPosition
        getStartPosition
        getMatrix
        getFullMatrix
        getGameStatus
        getKeys
        getCollectedTokens
        getPlayerPayments
        getMvpPercent
        getPlayers
        getPaymentInfo
        getMvpClaimed
        getPrizesToWin
        getStatus
        getMvp
        getGeneralData
        getTimer
        getDuration
        getMoves
        getCanMakeMove
        getPongAddress
        getAddressesWithMove
        registerPaymentToken
        loadTokens
        unloadTokens
        setMatrix
        clearMatrix
        setGameStatus
        setMvpPercent
        setInitPosition
        setPongAddress
        setStartPosition
        setPlayerPosition
        setDuration
        setFirstMoveTimestamp
        setStatus
        setCanMakeMove
        pause
        unpause
        isPaused
    )
}

multiversx_sc_wasm_adapter::empty_callback! {}