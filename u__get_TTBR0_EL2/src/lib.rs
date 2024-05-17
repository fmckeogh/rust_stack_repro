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
use Mk_TTBR0_EL2_Type::*;
use common::*;
pub fn u__get_TTBR0_EL2<T: Tracer>(
    state: &mut State,
    tracer: &T,
    value_name: ProductType782ac6922b48c20d,
) -> ProductType782ac6922b48c20d {
    #[derive(Default)]
    struct FunctionState {
        tmp: ProductType782ac6922b48c20d,
        value_name: ProductType782ac6922b48c20d,
    }
    let fn_state = FunctionState {
        value_name,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType782ac6922b48c20d {
        // D s_0_0: read-var value_name:struct
        let s_0_0: ProductType782ac6922b48c20d = fn_state.value_name;
        // D s_0_1: write-var tmp <= s_0_0
        fn_state.tmp = s_0_0;
        // D s_0_2: read-var tmp.0:struct
        let s_0_2: u128 = fn_state.tmp._0;
        // C s_0_3: const #0u : u8
        let s_0_3: u8 = 0;
        // C s_0_4: cast zx s_0_3 -> bv
        let s_0_4: Bits = Bits::new(s_0_3 as u128, 0u16);
        // C s_0_5: const #18446744073692839935u : u64
        let s_0_5: u64 = 18446744073692839935;
        // C s_0_6: cast zx s_0_5 -> bv
        let s_0_6: Bits = Bits::new(s_0_5 as u128, 64u16);
        // C s_0_7: cast reint s_0_4 -> u128
        let s_0_7: u128 = (s_0_4.value() as u128);
        // D s_0_8: size-of s_0_4
        let s_0_8: u16 = s_0_4.length();
        // C s_0_9: cast reint s_0_6 -> u128
        let s_0_9: u128 = (s_0_6.value() as u128);
        // D s_0_10: size-of s_0_6
        let s_0_10: u16 = s_0_6.length();
        // D s_0_11: lsl s_0_7 s_0_10
        let s_0_11: u128 = s_0_7 << s_0_10;
        // D s_0_12: or s_0_11 s_0_9
        let s_0_12: u128 = ((s_0_11) | (s_0_9));
        // D s_0_13: add s_0_8 s_0_10
        let s_0_13: u16 = (s_0_8 + s_0_10);
        // D s_0_14: create-bits s_0_12 s_0_13
        let s_0_14: Bits = Bits::new(s_0_12, s_0_13);
        // C s_0_15: const #0u : u64
        let s_0_15: u64 = 0;
        // C s_0_16: cast zx s_0_15 -> bv
        let s_0_16: Bits = Bits::new(s_0_15 as u128, 64u16);
        // D s_0_17: cast reint s_0_14 -> u128
        let s_0_17: u128 = (s_0_14.value() as u128);
        // D s_0_18: size-of s_0_14
        let s_0_18: u16 = s_0_14.length();
        // C s_0_19: cast reint s_0_16 -> u128
        let s_0_19: u128 = (s_0_16.value() as u128);
        // D s_0_20: size-of s_0_16
        let s_0_20: u16 = s_0_16.length();
        // D s_0_21: lsl s_0_17 s_0_20
        let s_0_21: u128 = s_0_17 << s_0_20;
        // D s_0_22: or s_0_21 s_0_19
        let s_0_22: u128 = ((s_0_21) | (s_0_19));
        // D s_0_23: add s_0_18 s_0_20
        let s_0_23: u16 = (s_0_18 + s_0_20);
        // D s_0_24: create-bits s_0_22 s_0_23
        let s_0_24: Bits = Bits::new(s_0_22, s_0_23);
        // D s_0_25: not s_0_24
        let s_0_25: Bits = !s_0_24;
        // D s_0_26: cast reint s_0_25 -> u128
        let s_0_26: u128 = (s_0_25.value() as u128);
        // D s_0_27: cast zx s_0_2 -> bv
        let s_0_27: Bits = Bits::new(s_0_2 as u128, 128u16);
        // D s_0_28: cast zx s_0_26 -> bv
        let s_0_28: Bits = Bits::new(s_0_26 as u128, 128u16);
        // D s_0_29: and s_0_27 s_0_28
        let s_0_29: Bits = ((s_0_27) & (s_0_28));
        // D s_0_30: cast reint s_0_29 -> u128
        let s_0_30: u128 = (s_0_29.value() as u128);
        // D s_0_31: tail-call Mk_TTBR0_EL2_Type(s_0_30)
        return Mk_TTBR0_EL2_Type(state, tracer, s_0_30);
    }
}
