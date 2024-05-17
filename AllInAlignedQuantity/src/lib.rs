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
use Align_bits::*;
use common::*;
pub fn AllInAlignedQuantity<T: Tracer>(
    state: &mut State,
    tracer: &T,
    address: u64,
    size: i128,
    alignment: i128,
) -> bool {
    #[derive(Default)]
    struct FunctionState {
        address: u64,
        size: i128,
        alignment: i128,
    }
    let fn_state = FunctionState {
        address,
        size,
        alignment,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_0_0: read-var size:i
        let s_0_0: i128 = fn_state.size;
        // D s_0_1: read-var alignment:i
        let s_0_1: i128 = fn_state.alignment;
        // D s_0_2: cmp-le s_0_0 s_0_1
        let s_0_2: bool = ((s_0_0) <= (s_0_1));
        // N s_0_3: assert s_0_2
        let s_0_3: () = assert!(s_0_2);
        // D s_0_4: read-var address:u64
        let s_0_4: u64 = fn_state.address;
        // D s_0_5: cast zx s_0_4 -> bv
        let s_0_5: Bits = Bits::new(s_0_4 as u128, 64u16);
        // D s_0_6: read-var size:i
        let s_0_6: i128 = fn_state.size;
        // D s_0_7: cast cvt s_0_6 -> bv
        let s_0_7: Bits = Bits::new(s_0_6 as u128, 128);
        // D s_0_8: add s_0_5 s_0_7
        let s_0_8: Bits = (s_0_5 + s_0_7);
        // D s_0_9: cast reint s_0_8 -> u64
        let s_0_9: u64 = (s_0_8.value() as u64);
        // C s_0_10: const #1s : i
        let s_0_10: i128 = 1;
        // D s_0_11: cast zx s_0_9 -> bv
        let s_0_11: Bits = Bits::new(s_0_9 as u128, 64u16);
        // C s_0_12: cast cvt s_0_10 -> bv
        let s_0_12: Bits = Bits::new(s_0_10 as u128, 128);
        // D s_0_13: sub s_0_11 s_0_12
        let s_0_13: Bits = ((s_0_11) - (s_0_12));
        // D s_0_14: cast reint s_0_13 -> u64
        let s_0_14: u64 = (s_0_13.value() as u64);
        // D s_0_15: cast zx s_0_14 -> bv
        let s_0_15: Bits = Bits::new(s_0_14 as u128, 64u16);
        // D s_0_16: read-var alignment:i
        let s_0_16: i128 = fn_state.alignment;
        // D s_0_17: call Align_bits(s_0_15, s_0_16)
        let s_0_17: Bits = Align_bits(state, tracer, s_0_15, s_0_16);
        // D s_0_18: cast reint s_0_17 -> u64
        let s_0_18: u64 = (s_0_17.value() as u64);
        // D s_0_19: read-var address:u64
        let s_0_19: u64 = fn_state.address;
        // D s_0_20: cast zx s_0_19 -> bv
        let s_0_20: Bits = Bits::new(s_0_19 as u128, 64u16);
        // D s_0_21: read-var alignment:i
        let s_0_21: i128 = fn_state.alignment;
        // D s_0_22: call Align_bits(s_0_20, s_0_21)
        let s_0_22: Bits = Align_bits(state, tracer, s_0_20, s_0_21);
        // D s_0_23: cast reint s_0_22 -> u64
        let s_0_23: u64 = (s_0_22.value() as u64);
        // D s_0_24: cast zx s_0_18 -> bv
        let s_0_24: Bits = Bits::new(s_0_18 as u128, 64u16);
        // D s_0_25: cast zx s_0_23 -> bv
        let s_0_25: Bits = Bits::new(s_0_23 as u128, 64u16);
        // D s_0_26: cmp-eq s_0_24 s_0_25
        let s_0_26: bool = ((s_0_24) == (s_0_25));
        // N s_0_27: return s_0_26
        return s_0_26;
    }
}
