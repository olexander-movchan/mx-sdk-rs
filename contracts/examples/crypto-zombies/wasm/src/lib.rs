// Code generated by the multiversx-sc multi-contract system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Endpoints:                           17
// Async Callback:                       1
// Total number of exported functions:  19

#![no_std]
#![feature(alloc_error_handler, lang_items)]

multiversx_sc_wasm_adapter::allocator!();
multiversx_sc_wasm_adapter::panic_handler!();

multiversx_sc_wasm_adapter::endpoints! {
    cryptozombies
    (
        set_crypto_kitties_sc_address
        generate_random_dna
        create_random_zombie
        is_ready
        feed_on_kitty
        dna_digits
        zombies_count
        zombies
        zombie_owner
        crypto_kitties_sc_address
        cooldown_time
        owned_zombies
        level_up
        withdraw
        change_name
        change_dna
        attack
        callBack
    )
}
