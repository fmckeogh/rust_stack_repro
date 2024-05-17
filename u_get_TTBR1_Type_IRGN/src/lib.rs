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
use common::*;
pub fn u_get_TTBR1_Type_IRGN<T: Tracer>(
    state: &mut State,
    tracer: &T,
    v: ProductType5c790c8ef59cc8b2,
) -> u8 {
    #[derive(Default)]
    struct FunctionState {
        v: ProductType5c790c8ef59cc8b2,
    }
    let fn_state = FunctionState {
        v,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // D s_0_0: read-var v.0:struct
        let s_0_0: u64 = fn_state.v._0;
        // C s_0_1: const #6s : i
        let s_0_1: i128 = 6;
        // D s_0_2: cast zx s_0_0 -> bv
        let s_0_2: Bits = Bits::new(s_0_0 as u128, 64u16);
        // C s_0_3: const #1s : i64
        let s_0_3: i64 = 1;
        // C s_0_4: cast zx s_0_3 -> i
        let s_0_4: i128 = (i128::try_from(s_0_3).unwrap());
        // C s_0_5: const #0s : i
        let s_0_5: i128 = 0;
        // C s_0_6: add s_0_5 s_0_4
        let s_0_6: i128 = (s_0_5 + s_0_4);
        // D s_0_7: bit-extract s_0_2 s_0_1 s_0_6
        let s_0_7: Bits = (Bits::new(
            ((s_0_2) >> (s_0_1)).value(),
            u16::try_from(s_0_6).unwrap(),
        ));
        // D s_0_8: cast reint s_0_7 -> u8
        let s_0_8: bool = ((s_0_7.value()) != 0);
        // D s_0_9: read-var v.0:struct
        let s_0_9: u64 = fn_state.v._0;
        // C s_0_10: const #0s : i
        let s_0_10: i128 = 0;
        // D s_0_11: cast zx s_0_9 -> bv
        let s_0_11: Bits = Bits::new(s_0_9 as u128, 64u16);
        // C s_0_12: const #1s : i64
        let s_0_12: i64 = 1;
        // C s_0_13: cast zx s_0_12 -> i
        let s_0_13: i128 = (i128::try_from(s_0_12).unwrap());
        // C s_0_14: const #0s : i
        let s_0_14: i128 = 0;
        // C s_0_15: add s_0_14 s_0_13
        let s_0_15: i128 = (s_0_14 + s_0_13);
        // D s_0_16: bit-extract s_0_11 s_0_10 s_0_15
        let s_0_16: Bits = (Bits::new(
            ((s_0_11) >> (s_0_10)).value(),
            u16::try_from(s_0_15).unwrap(),
        ));
        // D s_0_17: cast reint s_0_16 -> u8
        let s_0_17: bool = ((s_0_16.value()) != 0);
        // D s_0_18: cast zx s_0_8 -> bv
        let s_0_18: Bits = Bits::new(s_0_8 as u128, 1u16);
        // D s_0_19: cast zx s_0_17 -> bv
        let s_0_19: Bits = Bits::new(s_0_17 as u128, 1u16);
        // D s_0_20: cast reint s_0_18 -> u128
        let s_0_20: u128 = (s_0_18.value() as u128);
        // D s_0_21: size-of s_0_18
        let s_0_21: u16 = s_0_18.length();
        // D s_0_22: cast reint s_0_19 -> u128
        let s_0_22: u128 = (s_0_19.value() as u128);
        // D s_0_23: size-of s_0_19
        let s_0_23: u16 = s_0_19.length();
        // D s_0_24: lsl s_0_20 s_0_23
        let s_0_24: u128 = s_0_20 << s_0_23;
        // D s_0_25: or s_0_24 s_0_22
        let s_0_25: u128 = ((s_0_24) | (s_0_22));
        // D s_0_26: add s_0_21 s_0_23
        let s_0_26: u16 = (s_0_21 + s_0_23);
        // D s_0_27: create-bits s_0_25 s_0_26
        let s_0_27: Bits = Bits::new(s_0_25, s_0_26);
        // D s_0_28: cast reint s_0_27 -> u8
        let s_0_28: u8 = (s_0_27.value() as u8);
        // N s_0_29: return s_0_28
        return s_0_28;
    }
}
