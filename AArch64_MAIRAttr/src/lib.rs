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
use HaveAIEExt::*;
use u__id::*;
use common::*;
pub fn AArch64_MAIRAttr<T: Tracer>(
    state: &mut State,
    tracer: &T,
    index: i128,
    mair2: ProductType5c790c8ef59cc8b2,
    mair: ProductType5c790c8ef59cc8b2,
) -> u8 {
    #[derive(Default)]
    struct FunctionState {
        bit_indexshadow_302: i128,
        return_value: u8,
        gs_17859: bool,
        gs_17858: bool,
        gs_17868: bool,
        gs_17867: bool,
        gs_17879: bool,
        bit_index: i128,
        gs_17878: bool,
        bit_indexshadow_303: i128,
        index: i128,
        mair2: ProductType5c790c8ef59cc8b2,
        mair: ProductType5c790c8ef59cc8b2,
    }
    let fn_state = FunctionState {
        index,
        mair2,
        mair,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_0_0: const #8s : i
        let s_0_0: i128 = 8;
        // D s_0_1: read-var index:i
        let s_0_1: i128 = fn_state.index;
        // D s_0_2: mul s_0_0 s_0_1
        let s_0_2: i128 = ((s_0_0) * (s_0_1));
        // D s_0_3: write-var bit_index <= s_0_2
        fn_state.bit_index = s_0_2;
        // C s_0_4: const #8s : i
        let s_0_4: i128 = 8;
        // D s_0_5: read-var index:i
        let s_0_5: i128 = fn_state.index;
        // D s_0_6: cmp-lt s_0_5 s_0_4
        let s_0_6: bool = ((s_0_5) < (s_0_4));
        // N s_0_7: branch s_0_6 b21 b1
        if s_0_6 {
            return block_21(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_1_0: const #() : ()
        let s_1_0: () = ();
        // S s_1_1: call HaveAIEExt(s_1_0)
        let s_1_1: bool = HaveAIEExt(state, tracer, s_1_0);
        // N s_1_2: branch s_1_1 b20 b2
        if s_1_1 {
            return block_20(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_2_0: const #0u : u8
        let s_2_0: bool = false;
        // D s_2_1: write-var gs#17858 <= s_2_0
        fn_state.gs_17858 = s_2_0;
        // N s_2_2: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // D s_3_0: read-var gs#17858:u8
        let s_3_0: bool = fn_state.gs_17858;
        // D s_3_1: write-var gs#17859 <= s_3_0
        fn_state.gs_17859 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // D s_4_0: read-var gs#17859:u8
        let s_4_0: bool = fn_state.gs_17859;
        // N s_4_1: assert s_4_0
        let s_4_1: () = assert!(s_4_0);
        // C s_4_2: const #7s : i
        let s_4_2: i128 = 7;
        // D s_4_3: read-var index:i
        let s_4_3: i128 = fn_state.index;
        // D s_4_4: cmp-gt s_4_3 s_4_2
        let s_4_4: bool = ((s_4_3) > (s_4_2));
        // N s_4_5: branch s_4_4 b13 b5
        if s_4_4 {
            return block_13(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // D s_5_0: read-var bit_index:i
        let s_5_0: i128 = fn_state.bit_index;
        // D s_5_1: write-var bit_indexshadow#302 <= s_5_0
        fn_state.bit_indexshadow_302 = s_5_0;
        // D s_5_2: read-var bit_indexshadow#302:i
        let s_5_2: i128 = fn_state.bit_indexshadow_302;
        // D s_5_3: call __id(s_5_2)
        let s_5_3: i128 = u__id(state, tracer, s_5_2);
        // C s_5_4: const #0s : i
        let s_5_4: i128 = 0;
        // D s_5_5: cmp-le s_5_4 s_5_3
        let s_5_5: bool = ((s_5_4) <= (s_5_3));
        // N s_5_6: branch s_5_5 b9 b6
        if s_5_5 {
            return block_9(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_6_0: const #0u : u8
        let s_6_0: bool = false;
        // D s_6_1: write-var gs#17868 <= s_6_0
        fn_state.gs_17868 = s_6_0;
        // N s_6_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // D s_7_0: read-var gs#17868:u8
        let s_7_0: bool = fn_state.gs_17868;
        // N s_7_1: assert s_7_0
        let s_7_1: () = assert!(s_7_0);
        // D s_7_2: read-var mair.0:struct
        let s_7_2: u64 = fn_state.mair._0;
        // C s_7_3: const #8s : i
        let s_7_3: i128 = 8;
        // D s_7_4: cast zx s_7_2 -> bv
        let s_7_4: Bits = Bits::new(s_7_2 as u128, 64u16);
        // D s_7_5: read-var bit_indexshadow#302:i
        let s_7_5: i128 = fn_state.bit_indexshadow_302;
        // D s_7_6: bit-extract s_7_4 s_7_5 s_7_3
        let s_7_6: Bits = (Bits::new(
            ((s_7_4) >> (s_7_5)).value(),
            u16::try_from(s_7_3).unwrap(),
        ));
        // D s_7_7: cast reint s_7_6 -> u8
        let s_7_7: u8 = (s_7_6.value() as u8);
        // D s_7_8: write-var return_value <= s_7_7
        fn_state.return_value = s_7_7;
        // N s_7_9: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // D s_8_0: read-var return_value:u8
        let s_8_0: u8 = fn_state.return_value;
        // N s_8_1: return s_8_0
        return s_8_0;
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // D s_9_0: read-var bit_indexshadow#302:i
        let s_9_0: i128 = fn_state.bit_indexshadow_302;
        // D s_9_1: call __id(s_9_0)
        let s_9_1: i128 = u__id(state, tracer, s_9_0);
        // D s_9_2: read-var bit_indexshadow#302:i
        let s_9_2: i128 = fn_state.bit_indexshadow_302;
        // D s_9_3: call __id(s_9_2)
        let s_9_3: i128 = u__id(state, tracer, s_9_2);
        // C s_9_4: const #7s : i
        let s_9_4: i128 = 7;
        // D s_9_5: add s_9_3 s_9_4
        let s_9_5: i128 = (s_9_3 + s_9_4);
        // D s_9_6: cmp-le s_9_1 s_9_5
        let s_9_6: bool = ((s_9_1) <= (s_9_5));
        // N s_9_7: branch s_9_6 b12 b10
        if s_9_6 {
            return block_12(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_10_0: const #0u : u8
        let s_10_0: bool = false;
        // D s_10_1: write-var gs#17867 <= s_10_0
        fn_state.gs_17867 = s_10_0;
        // N s_10_2: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // D s_11_0: read-var gs#17867:u8
        let s_11_0: bool = fn_state.gs_17867;
        // D s_11_1: write-var gs#17868 <= s_11_0
        fn_state.gs_17868 = s_11_0;
        // N s_11_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // D s_12_0: read-var bit_indexshadow#302:i
        let s_12_0: i128 = fn_state.bit_indexshadow_302;
        // D s_12_1: call __id(s_12_0)
        let s_12_1: i128 = u__id(state, tracer, s_12_0);
        // C s_12_2: const #7s : i
        let s_12_2: i128 = 7;
        // D s_12_3: add s_12_1 s_12_2
        let s_12_3: i128 = (s_12_1 + s_12_2);
        // C s_12_4: const #64s : i
        let s_12_4: i128 = 64;
        // D s_12_5: cmp-lt s_12_3 s_12_4
        let s_12_5: bool = ((s_12_3) < (s_12_4));
        // D s_12_6: write-var gs#17867 <= s_12_5
        fn_state.gs_17867 = s_12_5;
        // N s_12_7: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_13_0: const #64s : i
        let s_13_0: i128 = 64;
        // D s_13_1: read-var bit_index:i
        let s_13_1: i128 = fn_state.bit_index;
        // D s_13_2: sub s_13_1 s_13_0
        let s_13_2: i128 = ((s_13_1) - (s_13_0));
        // D s_13_3: write-var bit_indexshadow#303 <= s_13_2
        fn_state.bit_indexshadow_303 = s_13_2;
        // D s_13_4: read-var bit_indexshadow#303:i
        let s_13_4: i128 = fn_state.bit_indexshadow_303;
        // D s_13_5: call __id(s_13_4)
        let s_13_5: i128 = u__id(state, tracer, s_13_4);
        // C s_13_6: const #0s : i
        let s_13_6: i128 = 0;
        // D s_13_7: cmp-le s_13_6 s_13_5
        let s_13_7: bool = ((s_13_6) <= (s_13_5));
        // N s_13_8: branch s_13_7 b16 b14
        if s_13_7 {
            return block_16(state, tracer, fn_state);
        } else {
            return block_14(state, tracer, fn_state);
        };
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_14_0: const #0u : u8
        let s_14_0: bool = false;
        // D s_14_1: write-var gs#17879 <= s_14_0
        fn_state.gs_17879 = s_14_0;
        // N s_14_2: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // D s_15_0: read-var gs#17879:u8
        let s_15_0: bool = fn_state.gs_17879;
        // N s_15_1: assert s_15_0
        let s_15_1: () = assert!(s_15_0);
        // D s_15_2: read-var mair2.0:struct
        let s_15_2: u64 = fn_state.mair2._0;
        // C s_15_3: const #8s : i
        let s_15_3: i128 = 8;
        // D s_15_4: cast zx s_15_2 -> bv
        let s_15_4: Bits = Bits::new(s_15_2 as u128, 64u16);
        // D s_15_5: read-var bit_indexshadow#303:i
        let s_15_5: i128 = fn_state.bit_indexshadow_303;
        // D s_15_6: bit-extract s_15_4 s_15_5 s_15_3
        let s_15_6: Bits = (Bits::new(
            ((s_15_4) >> (s_15_5)).value(),
            u16::try_from(s_15_3).unwrap(),
        ));
        // D s_15_7: cast reint s_15_6 -> u8
        let s_15_7: u8 = (s_15_6.value() as u8);
        // D s_15_8: write-var return_value <= s_15_7
        fn_state.return_value = s_15_7;
        // N s_15_9: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // D s_16_0: read-var bit_indexshadow#303:i
        let s_16_0: i128 = fn_state.bit_indexshadow_303;
        // D s_16_1: call __id(s_16_0)
        let s_16_1: i128 = u__id(state, tracer, s_16_0);
        // D s_16_2: read-var bit_indexshadow#303:i
        let s_16_2: i128 = fn_state.bit_indexshadow_303;
        // D s_16_3: call __id(s_16_2)
        let s_16_3: i128 = u__id(state, tracer, s_16_2);
        // C s_16_4: const #7s : i
        let s_16_4: i128 = 7;
        // D s_16_5: add s_16_3 s_16_4
        let s_16_5: i128 = (s_16_3 + s_16_4);
        // D s_16_6: cmp-le s_16_1 s_16_5
        let s_16_6: bool = ((s_16_1) <= (s_16_5));
        // N s_16_7: branch s_16_6 b19 b17
        if s_16_6 {
            return block_19(state, tracer, fn_state);
        } else {
            return block_17(state, tracer, fn_state);
        };
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_17_0: const #0u : u8
        let s_17_0: bool = false;
        // D s_17_1: write-var gs#17878 <= s_17_0
        fn_state.gs_17878 = s_17_0;
        // N s_17_2: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // D s_18_0: read-var gs#17878:u8
        let s_18_0: bool = fn_state.gs_17878;
        // D s_18_1: write-var gs#17879 <= s_18_0
        fn_state.gs_17879 = s_18_0;
        // N s_18_2: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // D s_19_0: read-var bit_indexshadow#303:i
        let s_19_0: i128 = fn_state.bit_indexshadow_303;
        // D s_19_1: call __id(s_19_0)
        let s_19_1: i128 = u__id(state, tracer, s_19_0);
        // C s_19_2: const #7s : i
        let s_19_2: i128 = 7;
        // D s_19_3: add s_19_1 s_19_2
        let s_19_3: i128 = (s_19_1 + s_19_2);
        // C s_19_4: const #64s : i
        let s_19_4: i128 = 64;
        // D s_19_5: cmp-lt s_19_3 s_19_4
        let s_19_5: bool = ((s_19_3) < (s_19_4));
        // D s_19_6: write-var gs#17878 <= s_19_5
        fn_state.gs_17878 = s_19_5;
        // N s_19_7: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_20_0: const #16s : i
        let s_20_0: i128 = 16;
        // D s_20_1: read-var index:i
        let s_20_1: i128 = fn_state.index;
        // D s_20_2: cmp-lt s_20_1 s_20_0
        let s_20_2: bool = ((s_20_1) < (s_20_0));
        // D s_20_3: write-var gs#17858 <= s_20_2
        fn_state.gs_17858 = s_20_2;
        // N s_20_4: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_21_0: const #1u : u8
        let s_21_0: bool = true;
        // D s_21_1: write-var gs#17859 <= s_21_0
        fn_state.gs_17859 = s_21_0;
        // N s_21_2: jump b4
        return block_4(state, tracer, fn_state);
    }
}
