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
use fdiv_int::*;
use TGxGranuleBits::*;
use AArch64_IASize::*;
use common::*;
pub fn AArch64_S1StartLevel<T: Tracer>(
    state: &mut State,
    tracer: &T,
    walkparams: ProductTypeef284266e139aee2,
) -> i128 {
    #[derive(Default)]
    struct FunctionState {
        granulebits: i128,
        ga_13221: i64,
        s1startlevel: i128,
        iasize: i128,
        walkparams: ProductTypeef284266e139aee2,
    }
    let fn_state = FunctionState {
        walkparams,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // D s_0_0: read-var walkparams.37:struct
        let s_0_0: u8 = fn_state.walkparams._37;
        // D s_0_1: call AArch64_IASize(s_0_0)
        let s_0_1: i128 = AArch64_IASize(state, tracer, s_0_0);
        // D s_0_2: write-var iasize <= s_0_1
        fn_state.iasize = s_0_1;
        // D s_0_3: read-var walkparams.36:struct
        let s_0_3: u32 = fn_state.walkparams._36;
        // D s_0_4: call TGxGranuleBits(s_0_3)
        let s_0_4: i128 = TGxGranuleBits(state, tracer, s_0_3);
        // D s_0_5: write-var granulebits <= s_0_4
        fn_state.granulebits = s_0_4;
        // D s_0_6: read-var walkparams.3:struct
        let s_0_6: bool = fn_state.walkparams._3;
        // D s_0_7: cast zx s_0_6 -> bv
        let s_0_7: Bits = Bits::new(s_0_6 as u128, 1u16);
        // C s_0_8: const #1u : u8
        let s_0_8: bool = true;
        // C s_0_9: cast zx s_0_8 -> bv
        let s_0_9: Bits = Bits::new(s_0_8 as u128, 1u16);
        // D s_0_10: cmp-eq s_0_7 s_0_9
        let s_0_10: bool = ((s_0_7) == (s_0_9));
        // N s_0_11: branch s_0_10 b6 b1
        if s_0_10 {
            return block_6(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // C s_1_0: const #3s : i64
        let s_1_0: i64 = 3;
        // D s_1_1: write-var ga#13221 <= s_1_0
        fn_state.ga_13221 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // D s_2_0: read-var ga#13221:i64
        let s_2_0: i64 = fn_state.ga_13221;
        // D s_2_1: cast zx s_2_0 -> i
        let s_2_1: i128 = (i128::try_from(s_2_0).unwrap());
        // D s_2_2: read-var granulebits:i
        let s_2_2: i128 = fn_state.granulebits;
        // D s_2_3: sub s_2_2 s_2_1
        let s_2_3: i128 = ((s_2_2) - (s_2_1));
        // C s_2_4: const #1s : i
        let s_2_4: i128 = 1;
        // D s_2_5: read-var iasize:i
        let s_2_5: i128 = fn_state.iasize;
        // D s_2_6: sub s_2_5 s_2_4
        let s_2_6: i128 = ((s_2_5) - (s_2_4));
        // D s_2_7: read-var granulebits:i
        let s_2_7: i128 = fn_state.granulebits;
        // D s_2_8: sub s_2_6 s_2_7
        let s_2_8: i128 = ((s_2_6) - (s_2_7));
        // D s_2_9: call fdiv_int(s_2_8, s_2_3)
        let s_2_9: i128 = fdiv_int(state, tracer, s_2_8, s_2_3);
        // C s_2_10: const #800u : u32
        let s_2_10: u32 = 800;
        // D s_2_11: read-reg s_2_10:i64
        let s_2_11: i64 = {
            let value = state.read_register::<i64>(s_2_10 as isize);
            tracer.read_register(s_2_10 as isize, value);
            value
        };
        // D s_2_12: cast zx s_2_11 -> i
        let s_2_12: i128 = (i128::try_from(s_2_11).unwrap());
        // D s_2_13: sub s_2_12 s_2_9
        let s_2_13: i128 = ((s_2_12) - (s_2_9));
        // D s_2_14: write-var s1startlevel <= s_2_13
        fn_state.s1startlevel = s_2_13;
        // D s_2_15: read-var walkparams.3:struct
        let s_2_15: bool = fn_state.walkparams._3;
        // D s_2_16: cast zx s_2_15 -> bv
        let s_2_16: Bits = Bits::new(s_2_15 as u128, 1u16);
        // C s_2_17: const #1u : u8
        let s_2_17: bool = true;
        // C s_2_18: cast zx s_2_17 -> bv
        let s_2_18: Bits = Bits::new(s_2_17 as u128, 1u16);
        // D s_2_19: cmp-eq s_2_16 s_2_18
        let s_2_19: bool = ((s_2_16) == (s_2_18));
        // N s_2_20: branch s_2_19 b5 b3
        if s_2_19 {
            return block_5(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // N s_3_0: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // D s_4_0: read-var s1startlevel:i
        let s_4_0: i128 = fn_state.s1startlevel;
        // N s_4_1: return s_4_0
        return s_4_0;
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // D s_5_0: read-var walkparams.31:struct
        let s_5_0: u8 = fn_state.walkparams._31;
        // D s_5_1: cast zx s_5_0 -> bv
        let s_5_1: Bits = Bits::new(s_5_0 as u128, 2u16);
        // D s_5_2: cast zx s_5_1 -> i
        let s_5_2: i128 = (s_5_1.value() as i128);
        // D s_5_3: cast reint s_5_2 -> i64
        let s_5_3: i64 = (s_5_2 as i64);
        // D s_5_4: cast zx s_5_3 -> i
        let s_5_4: i128 = (i128::try_from(s_5_3).unwrap());
        // D s_5_5: read-var s1startlevel:i
        let s_5_5: i128 = fn_state.s1startlevel;
        // D s_5_6: add s_5_5 s_5_4
        let s_5_6: i128 = (s_5_5 + s_5_4);
        // D s_5_7: write-var s1startlevel <= s_5_6
        fn_state.s1startlevel = s_5_6;
        // N s_5_8: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // C s_6_0: const #4s : i64
        let s_6_0: i64 = 4;
        // D s_6_1: write-var ga#13221 <= s_6_0
        fn_state.ga_13221 = s_6_0;
        // N s_6_2: jump b2
        return block_2(state, tracer, fn_state);
    }
}
