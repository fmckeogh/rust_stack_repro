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
use Mk_OSDLR_EL1_Type::*;
use IsFeatureImplemented::*;
use common::*;
pub fn u__get_OSDLR_EL1<T: Tracer>(
    state: &mut State,
    tracer: &T,
    value_name: ProductType5c790c8ef59cc8b2,
) -> ProductType5c790c8ef59cc8b2 {
    #[derive(Default)]
    struct FunctionState {
        tmp: ProductType5c790c8ef59cc8b2,
        value_name: ProductType5c790c8ef59cc8b2,
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
    ) -> ProductType5c790c8ef59cc8b2 {
        // D s_0_0: read-var value_name:struct
        let s_0_0: ProductType5c790c8ef59cc8b2 = fn_state.value_name;
        // D s_0_1: write-var tmp <= s_0_0
        fn_state.tmp = s_0_0;
        // C s_0_2: const #18u : u32
        let s_0_2: u32 = 18;
        // S s_0_3: call IsFeatureImplemented(s_0_2)
        let s_0_3: bool = IsFeatureImplemented(state, tracer, s_0_2);
        // S s_0_4: not s_0_3
        let s_0_4: bool = !s_0_3;
        // N s_0_5: branch s_0_4 b3 b1
        if s_0_4 {
            return block_3(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // N s_1_0: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // D s_2_0: read-var tmp.0:struct
        let s_2_0: u64 = fn_state.tmp._0;
        // C s_2_1: const #18446744073709551614u : u64
        let s_2_1: u64 = 18446744073709551614;
        // C s_2_2: cast zx s_2_1 -> bv
        let s_2_2: Bits = Bits::new(s_2_1 as u128, 64u16);
        // C s_2_3: not s_2_2
        let s_2_3: Bits = !s_2_2;
        // C s_2_4: cast reint s_2_3 -> u64
        let s_2_4: u64 = (s_2_3.value() as u64);
        // D s_2_5: cast zx s_2_0 -> bv
        let s_2_5: Bits = Bits::new(s_2_0 as u128, 64u16);
        // C s_2_6: cast zx s_2_4 -> bv
        let s_2_6: Bits = Bits::new(s_2_4 as u128, 64u16);
        // D s_2_7: and s_2_5 s_2_6
        let s_2_7: Bits = ((s_2_5) & (s_2_6));
        // D s_2_8: cast reint s_2_7 -> u64
        let s_2_8: u64 = (s_2_7.value() as u64);
        // D s_2_9: tail-call Mk_OSDLR_EL1_Type(s_2_8)
        return Mk_OSDLR_EL1_Type(state, tracer, s_2_8);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // D s_3_0: read-var tmp.0:struct
        let s_3_0: u64 = fn_state.tmp._0;
        // C s_3_1: const #64s : i
        let s_3_1: i128 = 64;
        // C s_3_2: const #1u : u8
        let s_3_2: u8 = 1;
        // C s_3_3: cast zx s_3_2 -> bv
        let s_3_3: Bits = Bits::new(s_3_2 as u128, 4u16);
        // D s_3_4: bits-cast zx s_3_3 -> bv length s_3_1
        let s_3_4: Bits = s_3_3.zero_extend(s_3_1);
        // D s_3_5: cast reint s_3_4 -> u64
        let s_3_5: u64 = (s_3_4.value() as u64);
        // D s_3_6: cast zx s_3_5 -> bv
        let s_3_6: Bits = Bits::new(s_3_5 as u128, 64u16);
        // D s_3_7: not s_3_6
        let s_3_7: Bits = !s_3_6;
        // D s_3_8: cast reint s_3_7 -> u64
        let s_3_8: u64 = (s_3_7.value() as u64);
        // D s_3_9: cast zx s_3_0 -> bv
        let s_3_9: Bits = Bits::new(s_3_0 as u128, 64u16);
        // D s_3_10: cast zx s_3_8 -> bv
        let s_3_10: Bits = Bits::new(s_3_8 as u128, 64u16);
        // D s_3_11: and s_3_9 s_3_10
        let s_3_11: Bits = ((s_3_9) & (s_3_10));
        // D s_3_12: cast reint s_3_11 -> u64
        let s_3_12: u64 = (s_3_11.value() as u64);
        // D s_3_13: call Mk_OSDLR_EL1_Type(s_3_12)
        let s_3_13: ProductType5c790c8ef59cc8b2 = Mk_OSDLR_EL1_Type(
            state,
            tracer,
            s_3_12,
        );
        // D s_3_14: write-var tmp <= s_3_13
        fn_state.tmp = s_3_13;
        // N s_3_15: jump b2
        return block_2(state, tracer, fn_state);
    }
}
