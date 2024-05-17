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
use Mk_PMCR_EL0_Type::*;
use IsFeatureImplemented::*;
use u__IMPDEF_boolean::*;
use common::*;
pub fn u__get_PMCR_EL0<T: Tracer>(
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
        // D s_0_2: read-var tmp.0:struct
        let s_0_2: u64 = fn_state.tmp._0;
        // C s_0_3: const #18446744065119618310u : u64
        let s_0_3: u64 = 18446744065119618310;
        // C s_0_4: cast zx s_0_3 -> bv
        let s_0_4: Bits = Bits::new(s_0_3 as u128, 64u16);
        // C s_0_5: not s_0_4
        let s_0_5: Bits = !s_0_4;
        // C s_0_6: cast reint s_0_5 -> u64
        let s_0_6: u64 = (s_0_5.value() as u64);
        // D s_0_7: cast zx s_0_2 -> bv
        let s_0_7: Bits = Bits::new(s_0_2 as u128, 64u16);
        // C s_0_8: cast zx s_0_6 -> bv
        let s_0_8: Bits = Bits::new(s_0_6 as u128, 64u16);
        // D s_0_9: and s_0_7 s_0_8
        let s_0_9: Bits = ((s_0_7) & (s_0_8));
        // D s_0_10: cast reint s_0_9 -> u64
        let s_0_10: u64 = (s_0_9.value() as u64);
        // D s_0_11: call Mk_PMCR_EL0_Type(s_0_10)
        let s_0_11: ProductType5c790c8ef59cc8b2 = Mk_PMCR_EL0_Type(
            state,
            tracer,
            s_0_10,
        );
        // D s_0_12: write-var tmp <= s_0_11
        fn_state.tmp = s_0_11;
        // C s_0_13: const #"the implementation includes a PMU event export bus" : str
        let s_0_13: &'static str = "the implementation includes a PMU event export bus";
        // S s_0_14: call __IMPDEF_boolean(s_0_13)
        let s_0_14: bool = u__IMPDEF_boolean(state, tracer, s_0_13);
        // S s_0_15: not s_0_14
        let s_0_15: bool = !s_0_14;
        // N s_0_16: branch s_0_15 b6 b1
        if s_0_15 {
            return block_6(state, tracer, fn_state);
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
        // C s_2_0: const #162u : u32
        let s_2_0: u32 = 162;
        // S s_2_1: call IsFeatureImplemented(s_2_0)
        let s_2_1: bool = IsFeatureImplemented(state, tracer, s_2_0);
        // S s_2_2: not s_2_1
        let s_2_2: bool = !s_2_1;
        // S s_2_3: not s_2_2
        let s_2_3: bool = !s_2_2;
        // N s_2_4: branch s_2_3 b5 b3
        if s_2_3 {
            return block_5(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // N s_3_0: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // D s_4_0: read-var tmp:struct
        let s_4_0: ProductType5c790c8ef59cc8b2 = fn_state.tmp;
        // N s_4_1: return s_4_0
        return s_4_0;
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // D s_5_0: read-var tmp.0:struct
        let s_5_0: u64 = fn_state.tmp._0;
        // C s_5_1: const #64s : i
        let s_5_1: i128 = 64;
        // C s_5_2: const #4278190080u : u32
        let s_5_2: u32 = 4278190080;
        // C s_5_3: cast zx s_5_2 -> bv
        let s_5_3: Bits = Bits::new(s_5_2 as u128, 32u16);
        // D s_5_4: bits-cast zx s_5_3 -> bv length s_5_1
        let s_5_4: Bits = s_5_3.zero_extend(s_5_1);
        // D s_5_5: cast reint s_5_4 -> u64
        let s_5_5: u64 = (s_5_4.value() as u64);
        // D s_5_6: cast zx s_5_5 -> bv
        let s_5_6: Bits = Bits::new(s_5_5 as u128, 64u16);
        // D s_5_7: not s_5_6
        let s_5_7: Bits = !s_5_6;
        // D s_5_8: cast reint s_5_7 -> u64
        let s_5_8: u64 = (s_5_7.value() as u64);
        // D s_5_9: cast zx s_5_0 -> bv
        let s_5_9: Bits = Bits::new(s_5_0 as u128, 64u16);
        // D s_5_10: cast zx s_5_8 -> bv
        let s_5_10: Bits = Bits::new(s_5_8 as u128, 64u16);
        // D s_5_11: and s_5_9 s_5_10
        let s_5_11: Bits = ((s_5_9) & (s_5_10));
        // D s_5_12: cast reint s_5_11 -> u64
        let s_5_12: u64 = (s_5_11.value() as u64);
        // D s_5_13: call Mk_PMCR_EL0_Type(s_5_12)
        let s_5_13: ProductType5c790c8ef59cc8b2 = Mk_PMCR_EL0_Type(
            state,
            tracer,
            s_5_12,
        );
        // D s_5_14: write-var tmp <= s_5_13
        fn_state.tmp = s_5_13;
        // N s_5_15: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // D s_6_0: read-var tmp.0:struct
        let s_6_0: u64 = fn_state.tmp._0;
        // C s_6_1: const #64s : i
        let s_6_1: i128 = 64;
        // C s_6_2: const #16u : u8
        let s_6_2: u8 = 16;
        // C s_6_3: cast zx s_6_2 -> bv
        let s_6_3: Bits = Bits::new(s_6_2 as u128, 8u16);
        // D s_6_4: bits-cast zx s_6_3 -> bv length s_6_1
        let s_6_4: Bits = s_6_3.zero_extend(s_6_1);
        // D s_6_5: cast reint s_6_4 -> u64
        let s_6_5: u64 = (s_6_4.value() as u64);
        // D s_6_6: cast zx s_6_5 -> bv
        let s_6_6: Bits = Bits::new(s_6_5 as u128, 64u16);
        // D s_6_7: not s_6_6
        let s_6_7: Bits = !s_6_6;
        // D s_6_8: cast reint s_6_7 -> u64
        let s_6_8: u64 = (s_6_7.value() as u64);
        // D s_6_9: cast zx s_6_0 -> bv
        let s_6_9: Bits = Bits::new(s_6_0 as u128, 64u16);
        // D s_6_10: cast zx s_6_8 -> bv
        let s_6_10: Bits = Bits::new(s_6_8 as u128, 64u16);
        // D s_6_11: and s_6_9 s_6_10
        let s_6_11: Bits = ((s_6_9) & (s_6_10));
        // D s_6_12: cast reint s_6_11 -> u64
        let s_6_12: u64 = (s_6_11.value() as u64);
        // D s_6_13: call Mk_PMCR_EL0_Type(s_6_12)
        let s_6_13: ProductType5c790c8ef59cc8b2 = Mk_PMCR_EL0_Type(
            state,
            tracer,
            s_6_12,
        );
        // D s_6_14: write-var tmp <= s_6_13
        fn_state.tmp = s_6_13;
        // N s_6_15: jump b2
        return block_2(state, tracer, fn_state);
    }
}
