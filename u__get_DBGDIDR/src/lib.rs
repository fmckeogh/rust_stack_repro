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
use Mk_DBGDIDR_Type::*;
use common::*;
pub fn u__get_DBGDIDR<T: Tracer>(
    state: &mut State,
    tracer: &T,
    value_name: ProductType700c18a878c5601b,
) -> ProductType700c18a878c5601b {
    #[derive(Default)]
    struct FunctionState {
        tmp: ProductType700c18a878c5601b,
        value_name: ProductType700c18a878c5601b,
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
    ) -> ProductType700c18a878c5601b {
        // D s_0_0: read-var value_name:struct
        let s_0_0: ProductType700c18a878c5601b = fn_state.value_name;
        // D s_0_1: write-var tmp <= s_0_0
        fn_state.tmp = s_0_0;
        // D s_0_2: read-var tmp.0:struct
        let s_0_2: u32 = fn_state.tmp._0;
        // C s_0_3: const #32s : i
        let s_0_3: i128 = 32;
        // C s_0_4: const #12287u : u16
        let s_0_4: u16 = 12287;
        // C s_0_5: cast zx s_0_4 -> bv
        let s_0_5: Bits = Bits::new(s_0_4 as u128, 16u16);
        // D s_0_6: bits-cast zx s_0_5 -> bv length s_0_3
        let s_0_6: Bits = s_0_5.zero_extend(s_0_3);
        // D s_0_7: cast reint s_0_6 -> u32
        let s_0_7: u32 = (s_0_6.value() as u32);
        // D s_0_8: cast zx s_0_7 -> bv
        let s_0_8: Bits = Bits::new(s_0_7 as u128, 32u16);
        // D s_0_9: not s_0_8
        let s_0_9: Bits = !s_0_8;
        // D s_0_10: cast reint s_0_9 -> u32
        let s_0_10: u32 = (s_0_9.value() as u32);
        // D s_0_11: cast zx s_0_2 -> bv
        let s_0_11: Bits = Bits::new(s_0_2 as u128, 32u16);
        // D s_0_12: cast zx s_0_10 -> bv
        let s_0_12: Bits = Bits::new(s_0_10 as u128, 32u16);
        // D s_0_13: and s_0_11 s_0_12
        let s_0_13: Bits = ((s_0_11) & (s_0_12));
        // D s_0_14: cast reint s_0_13 -> u32
        let s_0_14: u32 = (s_0_13.value() as u32);
        // C s_0_15: const #32s : i
        let s_0_15: i128 = 32;
        // C s_0_16: const #32768u : u16
        let s_0_16: u16 = 32768;
        // C s_0_17: cast zx s_0_16 -> bv
        let s_0_17: Bits = Bits::new(s_0_16 as u128, 16u16);
        // D s_0_18: bits-cast zx s_0_17 -> bv length s_0_15
        let s_0_18: Bits = s_0_17.zero_extend(s_0_15);
        // D s_0_19: cast reint s_0_18 -> u32
        let s_0_19: u32 = (s_0_18.value() as u32);
        // D s_0_20: cast zx s_0_14 -> bv
        let s_0_20: Bits = Bits::new(s_0_14 as u128, 32u16);
        // D s_0_21: cast zx s_0_19 -> bv
        let s_0_21: Bits = Bits::new(s_0_19 as u128, 32u16);
        // D s_0_22: or s_0_20 s_0_21
        let s_0_22: Bits = ((s_0_20) | (s_0_21));
        // D s_0_23: cast reint s_0_22 -> u32
        let s_0_23: u32 = (s_0_22.value() as u32);
        // D s_0_24: tail-call Mk_DBGDIDR_Type(s_0_23)
        return Mk_DBGDIDR_Type(state, tracer, s_0_23);
    }
}
