#![no_std]
#![allow(non_snake_case)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![allow(unused_parens)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unreachable_code)]
#![allow(unused_doc_comments)]
#![allow(non_upper_case_globals)]
//! BOREALIS GENERATED FILE
extern crate alloc;
use Bit::*;
use CurrentInstrSet::*;
use BranchTo::*;
use common::*;
pub fn CBWritePC<T: Tracer>(state: &mut State, tracer: &T, address_in: u32) -> () {
    #[derive(Default)]
    struct FunctionState {
        address_in: u32,
    }
    let fn_state = FunctionState {
        address_in,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_0_0: read-var address_in:u32
        let s_0_0: u32 = fn_state.address_in;
        // C s_0_1: const #() : ()
        let s_0_1: () = ();
        // S s_0_2: call CurrentInstrSet(s_0_1)
        let s_0_2: u32 = CurrentInstrSet(state, tracer, s_0_1);
        // C s_0_3: const #2u : u32
        let s_0_3: u32 = 2;
        // S s_0_4: cmp-eq s_0_2 s_0_3
        let s_0_4: bool = ((s_0_2) == (s_0_3));
        // N s_0_5: assert s_0_4
        let s_0_5: () = assert!(s_0_4);
        // C s_0_6: const #0u : u8
        let s_0_6: bool = false;
        // S s_0_7: call Bit(s_0_6)
        let s_0_7: bool = Bit(state, tracer, s_0_6);
        // C s_0_8: const #0s : i
        let s_0_8: i128 = 0;
        // D s_0_9: cast zx s_0_0 -> bv
        let s_0_9: Bits = Bits::new(s_0_0 as u128, 32u16);
        // C s_0_10: const #1u : u64
        let s_0_10: u64 = 1;
        // D s_0_11: bit-insert s_0_9 s_0_9 s_0_8 s_0_10
        let s_0_11: Bits = {
            let mask = !Bits::new(
                ((1u128).checked_shl(s_0_10 as u32).map(|x| x - 1).unwrap_or(!0)),
                s_0_9.length(),
            );
            (s_0_9 & mask) | (s_0_9 << s_0_8)
        };
        // D s_0_12: cast reint s_0_11 -> u32
        let s_0_12: u32 = (s_0_11.value() as u32);
        // C s_0_13: const #1u : u8
        let s_0_13: bool = true;
        // D s_0_14: cast zx s_0_12 -> bv
        let s_0_14: Bits = Bits::new(s_0_12 as u128, 32u16);
        // C s_0_15: const #5u : u32
        let s_0_15: u32 = 5;
        // D s_0_16: call BranchTo(s_0_14, s_0_15, s_0_13)
        let s_0_16: () = BranchTo(state, tracer, s_0_14, s_0_15, s_0_13);
        // N s_0_17: return
        return;
    }
}
