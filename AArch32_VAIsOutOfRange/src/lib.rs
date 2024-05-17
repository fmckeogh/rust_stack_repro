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
use AArch32_S1IASize::*;
use is_zero_subrange::*;
use is_ones_subrange::*;
use common::*;
pub fn AArch32_VAIsOutOfRange<T: Tracer>(
    state: &mut State,
    tracer: &T,
    regime: u32,
    walkparams: ProductTypeef284266e139aee2,
    va: u32,
) -> bool {
    #[derive(Default)]
    struct FunctionState {
        gs_28381: bool,
        iasizeshadow_527: i128,
        return_value: bool,
        up_iasizeshadow_526: i128,
        gs_28386: bool,
        gs_28391: bool,
        regime: u32,
        walkparams: ProductTypeef284266e139aee2,
        va: u32,
    }
    let fn_state = FunctionState {
        regime,
        walkparams,
        va,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_0_0: read-var regime:u32
        let s_0_0: u32 = fn_state.regime;
        // C s_0_1: const #2u : u32
        let s_0_1: u32 = 2;
        // D s_0_2: cmp-eq s_0_0 s_0_1
        let s_0_2: bool = ((s_0_0) == (s_0_1));
        // N s_0_3: branch s_0_2 b11 b1
        if s_0_2 {
            return block_11(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_1_0: read-var walkparams.33:struct
        let s_1_0: u8 = fn_state.walkparams._33;
        // D s_1_1: cast zx s_1_0 -> bv
        let s_1_1: Bits = Bits::new(s_1_0 as u128, 3u16);
        // C s_1_2: const #0u : u8
        let s_1_2: u8 = 0;
        // C s_1_3: cast zx s_1_2 -> bv
        let s_1_3: Bits = Bits::new(s_1_2 as u128, 3u16);
        // D s_1_4: cmp-ne s_1_1 s_1_3
        let s_1_4: bool = ((s_1_1) != (s_1_3));
        // N s_1_5: branch s_1_4 b10 b2
        if s_1_4 {
            return block_10(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_2_0: const #0u : u8
        let s_2_0: bool = false;
        // D s_2_1: write-var gs#28381 <= s_2_0
        fn_state.gs_28381 = s_2_0;
        // N s_2_2: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_3_0: read-var gs#28381:u8
        let s_3_0: bool = fn_state.gs_28381;
        // N s_3_1: branch s_3_0 b6 b4
        if s_3_0 {
            return block_6(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_4_0: const #0u : u8
        let s_4_0: bool = false;
        // D s_4_1: write-var return_value <= s_4_0
        fn_state.return_value = s_4_0;
        // N s_4_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_5_0: read-var return_value:u8
        let s_5_0: bool = fn_state.return_value;
        // N s_5_1: return s_5_0
        return s_5_0;
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_6_0: read-var walkparams.32:struct
        let s_6_0: u8 = fn_state.walkparams._32;
        // D s_6_1: call AArch32_S1IASize(s_6_0)
        let s_6_1: i128 = AArch32_S1IASize(state, tracer, s_6_0);
        // D s_6_2: read-var walkparams.33:struct
        let s_6_2: u8 = fn_state.walkparams._33;
        // D s_6_3: call AArch32_S1IASize(s_6_2)
        let s_6_3: i128 = AArch32_S1IASize(state, tracer, s_6_2);
        // D s_6_4: write-var up_iasizeshadow#526 <= s_6_3
        fn_state.up_iasizeshadow_526 = s_6_3;
        // C s_6_5: const #31s : i
        let s_6_5: i128 = 31;
        // D s_6_6: read-var va:u32
        let s_6_6: u32 = fn_state.va;
        // D s_6_7: cast zx s_6_6 -> bv
        let s_6_7: Bits = Bits::new(s_6_6 as u128, 32u16);
        // D s_6_8: call is_zero_subrange(s_6_7, s_6_5, s_6_1)
        let s_6_8: bool = is_zero_subrange(state, tracer, s_6_7, s_6_5, s_6_1);
        // D s_6_9: not s_6_8
        let s_6_9: bool = !s_6_8;
        // N s_6_10: branch s_6_9 b9 b7
        if s_6_9 {
            return block_9(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_7_0: const #0u : u8
        let s_7_0: bool = false;
        // D s_7_1: write-var gs#28386 <= s_7_0
        fn_state.gs_28386 = s_7_0;
        // N s_7_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_8_0: read-var gs#28386:u8
        let s_8_0: bool = fn_state.gs_28386;
        // D s_8_1: write-var return_value <= s_8_0
        fn_state.return_value = s_8_0;
        // N s_8_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_9_0: const #31s : i
        let s_9_0: i128 = 31;
        // D s_9_1: read-var va:u32
        let s_9_1: u32 = fn_state.va;
        // D s_9_2: cast zx s_9_1 -> bv
        let s_9_2: Bits = Bits::new(s_9_1 as u128, 32u16);
        // D s_9_3: read-var up_iasizeshadow#526:i
        let s_9_3: i128 = fn_state.up_iasizeshadow_526;
        // D s_9_4: call is_ones_subrange(s_9_2, s_9_0, s_9_3)
        let s_9_4: bool = is_ones_subrange(state, tracer, s_9_2, s_9_0, s_9_3);
        // D s_9_5: not s_9_4
        let s_9_5: bool = !s_9_4;
        // D s_9_6: write-var gs#28386 <= s_9_5
        fn_state.gs_28386 = s_9_5;
        // N s_9_7: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_10_0: read-var walkparams.32:struct
        let s_10_0: u8 = fn_state.walkparams._32;
        // D s_10_1: cast zx s_10_0 -> bv
        let s_10_1: Bits = Bits::new(s_10_0 as u128, 3u16);
        // C s_10_2: const #0u : u8
        let s_10_2: u8 = 0;
        // C s_10_3: cast zx s_10_2 -> bv
        let s_10_3: Bits = Bits::new(s_10_2 as u128, 3u16);
        // D s_10_4: cmp-ne s_10_1 s_10_3
        let s_10_4: bool = ((s_10_1) != (s_10_3));
        // D s_10_5: write-var gs#28381 <= s_10_4
        fn_state.gs_28381 = s_10_4;
        // N s_10_6: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_11_0: read-var walkparams.32:struct
        let s_11_0: u8 = fn_state.walkparams._32;
        // D s_11_1: call AArch32_S1IASize(s_11_0)
        let s_11_1: i128 = AArch32_S1IASize(state, tracer, s_11_0);
        // D s_11_2: write-var iasizeshadow#527 <= s_11_1
        fn_state.iasizeshadow_527 = s_11_1;
        // D s_11_3: read-var walkparams.32:struct
        let s_11_3: u8 = fn_state.walkparams._32;
        // D s_11_4: cast zx s_11_3 -> bv
        let s_11_4: Bits = Bits::new(s_11_3 as u128, 3u16);
        // C s_11_5: const #0u : u8
        let s_11_5: u8 = 0;
        // C s_11_6: cast zx s_11_5 -> bv
        let s_11_6: Bits = Bits::new(s_11_5 as u128, 3u16);
        // D s_11_7: cmp-ne s_11_4 s_11_6
        let s_11_7: bool = ((s_11_4) != (s_11_6));
        // N s_11_8: branch s_11_7 b14 b12
        if s_11_7 {
            return block_14(state, tracer, fn_state);
        } else {
            return block_12(state, tracer, fn_state);
        };
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_12_0: const #0u : u8
        let s_12_0: bool = false;
        // D s_12_1: write-var gs#28391 <= s_12_0
        fn_state.gs_28391 = s_12_0;
        // N s_12_2: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_13_0: read-var gs#28391:u8
        let s_13_0: bool = fn_state.gs_28391;
        // D s_13_1: write-var return_value <= s_13_0
        fn_state.return_value = s_13_0;
        // N s_13_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_14_0: const #31s : i
        let s_14_0: i128 = 31;
        // D s_14_1: read-var va:u32
        let s_14_1: u32 = fn_state.va;
        // D s_14_2: cast zx s_14_1 -> bv
        let s_14_2: Bits = Bits::new(s_14_1 as u128, 32u16);
        // D s_14_3: read-var iasizeshadow#527:i
        let s_14_3: i128 = fn_state.iasizeshadow_527;
        // D s_14_4: call is_zero_subrange(s_14_2, s_14_0, s_14_3)
        let s_14_4: bool = is_zero_subrange(state, tracer, s_14_2, s_14_0, s_14_3);
        // D s_14_5: not s_14_4
        let s_14_5: bool = !s_14_4;
        // D s_14_6: write-var gs#28391 <= s_14_5
        fn_state.gs_28391 = s_14_5;
        // N s_14_7: jump b13
        return block_13(state, tracer, fn_state);
    }
}
