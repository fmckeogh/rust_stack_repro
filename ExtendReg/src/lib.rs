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
use place_subrange_signed::*;
use X_read::*;
use u__id::*;
use place_subrange::*;
use common::*;
pub fn ExtendReg<T: Tracer>(
    state: &mut State,
    tracer: &T,
    reg: i128,
    exttype: u32,
    shift: i128,
    N: i128,
) -> Bits {
    #[derive(Default)]
    struct FunctionState {
        ga_20627: Bits,
        len: i128,
        is_unsignedshadow_498: bool,
        gs_26460: bool,
        is_unsigned: bool,
        gs_26459: bool,
        val_name: Bits,
        gs_26426: bool,
        lenshadow_499: i128,
        Nshadow_497: i128,
        reg: i128,
        exttype: u32,
        shift: i128,
        N: i128,
    }
    let fn_state = FunctionState {
        reg,
        exttype,
        shift,
        N,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_0_0: read-var N:i
        let s_0_0: i128 = fn_state.N;
        // D s_0_1: write-var Nshadow#497 <= s_0_0
        fn_state.Nshadow_497 = s_0_0;
        // C s_0_2: const #0s : i
        let s_0_2: i128 = 0;
        // D s_0_3: read-var shift:i
        let s_0_3: i128 = fn_state.shift;
        // D s_0_4: cmp-ge s_0_3 s_0_2
        let s_0_4: bool = ((s_0_3) >= (s_0_2));
        // N s_0_5: branch s_0_4 b29 b1
        if s_0_4 {
            return block_29(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_1_0: const #0u : u8
        let s_1_0: bool = false;
        // D s_1_1: write-var gs#26426 <= s_1_0
        fn_state.gs_26426 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_2_0: read-var gs#26426:u8
        let s_2_0: bool = fn_state.gs_26426;
        // N s_2_1: assert s_2_0
        let s_2_1: () = assert!(s_2_0);
        // D s_2_2: read-var Nshadow#497:i
        let s_2_2: i128 = fn_state.Nshadow_497;
        // D s_2_3: cast reint s_2_2 -> i64
        let s_2_3: i64 = (s_2_2 as i64);
        // D s_2_4: read-var reg:i
        let s_2_4: i128 = fn_state.reg;
        // D s_2_5: call X_read(s_2_4, s_2_3)
        let s_2_5: Bits = X_read(state, tracer, s_2_4, s_2_3);
        // D s_2_6: write-var val_name <= s_2_5
        fn_state.val_name = s_2_5;
        // C s_2_7: const #0u : u32
        let s_2_7: u32 = 0;
        // D s_2_8: read-var exttype:u32
        let s_2_8: u32 = fn_state.exttype;
        // D s_2_9: cmp-eq s_2_7 s_2_8
        let s_2_9: bool = ((s_2_7) == (s_2_8));
        // D s_2_10: not s_2_9
        let s_2_10: bool = !s_2_9;
        // N s_2_11: branch s_2_10 b14 b3
        if s_2_10 {
            return block_14(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_3_0: const #0u : u8
        let s_3_0: bool = false;
        // D s_3_1: write-var is_unsigned <= s_3_0
        fn_state.is_unsigned = s_3_0;
        // C s_3_2: const #8s : i
        let s_3_2: i128 = 8;
        // D s_3_3: write-var len <= s_3_2
        fn_state.len = s_3_2;
        // N s_3_4: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_4_0: read-var is_unsigned:u8
        let s_4_0: bool = fn_state.is_unsigned;
        // D s_4_1: write-var is_unsignedshadow#498 <= s_4_0
        fn_state.is_unsignedshadow_498 = s_4_0;
        // D s_4_2: read-var Nshadow#497:i
        let s_4_2: i128 = fn_state.Nshadow_497;
        // D s_4_3: read-var shift:i
        let s_4_3: i128 = fn_state.shift;
        // D s_4_4: sub s_4_2 s_4_3
        let s_4_4: i128 = ((s_4_2) - (s_4_3));
        // D s_4_5: cast reint s_4_4 -> i64
        let s_4_5: i64 = (s_4_4 as i64);
        // D s_4_6: cast zx s_4_5 -> i
        let s_4_6: i128 = (i128::try_from(s_4_5).unwrap());
        // D s_4_7: read-var len:i
        let s_4_7: i128 = fn_state.len;
        // D s_4_8: cmp-lt s_4_7 s_4_6
        let s_4_8: bool = ((s_4_7) < (s_4_6));
        // D s_4_9: select s_4_8 s_4_7 s_4_6
        let s_4_9: i128 = if s_4_8 { s_4_7 } else { s_4_6 };
        // D s_4_10: write-var lenshadow#499 <= s_4_9
        fn_state.lenshadow_499 = s_4_9;
        // D s_4_11: read-var lenshadow#499:i
        let s_4_11: i128 = fn_state.lenshadow_499;
        // D s_4_12: call __id(s_4_11)
        let s_4_12: i128 = u__id(state, tracer, s_4_11);
        // C s_4_13: const #1s : i
        let s_4_13: i128 = 1;
        // D s_4_14: sub s_4_12 s_4_13
        let s_4_14: i128 = ((s_4_12) - (s_4_13));
        // C s_4_15: const #0s : i
        let s_4_15: i128 = 0;
        // D s_4_16: cmp-le s_4_15 s_4_14
        let s_4_16: bool = ((s_4_15) <= (s_4_14));
        // N s_4_17: branch s_4_16 b13 b5
        if s_4_16 {
            return block_13(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_5_0: read-var shift:i
        let s_5_0: i128 = fn_state.shift;
        // D s_5_1: call __id(s_5_0)
        let s_5_1: i128 = u__id(state, tracer, s_5_0);
        // D s_5_2: cast reint s_5_1 -> i64
        let s_5_2: i64 = (s_5_1 as i64);
        // C s_5_3: const #0s : i
        let s_5_3: i128 = 0;
        // D s_5_4: cast zx s_5_2 -> i
        let s_5_4: i128 = (i128::try_from(s_5_2).unwrap());
        // D s_5_5: cmp-ge s_5_4 s_5_3
        let s_5_5: bool = ((s_5_4) >= (s_5_3));
        // N s_5_6: branch s_5_5 b12 b6
        if s_5_5 {
            return block_12(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_6_0: const #0u : u8
        let s_6_0: bool = false;
        // D s_6_1: write-var gs#26459 <= s_6_0
        fn_state.gs_26459 = s_6_0;
        // N s_6_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_7_0: read-var gs#26459:u8
        let s_7_0: bool = fn_state.gs_26459;
        // D s_7_1: not s_7_0
        let s_7_1: bool = !s_7_0;
        // D s_7_2: write-var gs#26460 <= s_7_1
        fn_state.gs_26460 = s_7_1;
        // N s_7_3: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_8_0: read-var gs#26460:u8
        let s_8_0: bool = fn_state.gs_26460;
        // N s_8_1: assert s_8_0
        let s_8_1: () = assert!(s_8_0);
        // D s_8_2: read-var is_unsignedshadow#498:u8
        let s_8_2: bool = fn_state.is_unsignedshadow_498;
        // N s_8_3: branch s_8_2 b11 b9
        if s_8_2 {
            return block_11(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_9_0: const #1s : i
        let s_9_0: i128 = 1;
        // D s_9_1: read-var lenshadow#499:i
        let s_9_1: i128 = fn_state.lenshadow_499;
        // D s_9_2: sub s_9_1 s_9_0
        let s_9_2: i128 = ((s_9_1) - (s_9_0));
        // D s_9_3: cast reint s_9_2 -> i64
        let s_9_3: i64 = (s_9_2 as i64);
        // C s_9_4: const #0s : i
        let s_9_4: i128 = 0;
        // D s_9_5: cast zx s_9_3 -> i
        let s_9_5: i128 = (i128::try_from(s_9_3).unwrap());
        // D s_9_6: read-var Nshadow#497:i
        let s_9_6: i128 = fn_state.Nshadow_497;
        // D s_9_7: read-var val_name:bv
        let s_9_7: Bits = fn_state.val_name;
        // D s_9_8: read-var shift:i
        let s_9_8: i128 = fn_state.shift;
        // D s_9_9: call place_subrange_signed(s_9_6, s_9_7, s_9_5, s_9_4, s_9_8)
        let s_9_9: Bits = place_subrange_signed(
            state,
            tracer,
            s_9_6,
            s_9_7,
            s_9_5,
            s_9_4,
            s_9_8,
        );
        // D s_9_10: write-var ga#20627 <= s_9_9
        fn_state.ga_20627 = s_9_9;
        // N s_9_11: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_10_0: read-var ga#20627:bv
        let s_10_0: Bits = fn_state.ga_20627;
        // N s_10_1: return s_10_0
        return s_10_0;
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_11_0: const #1s : i
        let s_11_0: i128 = 1;
        // D s_11_1: read-var lenshadow#499:i
        let s_11_1: i128 = fn_state.lenshadow_499;
        // D s_11_2: sub s_11_1 s_11_0
        let s_11_2: i128 = ((s_11_1) - (s_11_0));
        // D s_11_3: cast reint s_11_2 -> i64
        let s_11_3: i64 = (s_11_2 as i64);
        // C s_11_4: const #0s : i
        let s_11_4: i128 = 0;
        // D s_11_5: cast zx s_11_3 -> i
        let s_11_5: i128 = (i128::try_from(s_11_3).unwrap());
        // D s_11_6: read-var Nshadow#497:i
        let s_11_6: i128 = fn_state.Nshadow_497;
        // D s_11_7: read-var val_name:bv
        let s_11_7: Bits = fn_state.val_name;
        // D s_11_8: read-var shift:i
        let s_11_8: i128 = fn_state.shift;
        // D s_11_9: call place_subrange(s_11_6, s_11_7, s_11_5, s_11_4, s_11_8)
        let s_11_9: Bits = place_subrange(
            state,
            tracer,
            s_11_6,
            s_11_7,
            s_11_5,
            s_11_4,
            s_11_8,
        );
        // D s_11_10: write-var ga#20627 <= s_11_9
        fn_state.ga_20627 = s_11_9;
        // N s_11_11: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_12_0: read-var shift:i
        let s_12_0: i128 = fn_state.shift;
        // D s_12_1: call __id(s_12_0)
        let s_12_1: i128 = u__id(state, tracer, s_12_0);
        // D s_12_2: cast reint s_12_1 -> i64
        let s_12_2: i64 = (s_12_1 as i64);
        // C s_12_3: const #4s : i
        let s_12_3: i128 = 4;
        // D s_12_4: cast zx s_12_2 -> i
        let s_12_4: i128 = (i128::try_from(s_12_2).unwrap());
        // D s_12_5: cmp-le s_12_4 s_12_3
        let s_12_5: bool = ((s_12_4) <= (s_12_3));
        // D s_12_6: write-var gs#26459 <= s_12_5
        fn_state.gs_26459 = s_12_5;
        // N s_12_7: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_13_0: const #1u : u8
        let s_13_0: bool = true;
        // D s_13_1: write-var gs#26460 <= s_13_0
        fn_state.gs_26460 = s_13_0;
        // N s_13_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_14_0: const #1u : u32
        let s_14_0: u32 = 1;
        // D s_14_1: read-var exttype:u32
        let s_14_1: u32 = fn_state.exttype;
        // D s_14_2: cmp-eq s_14_0 s_14_1
        let s_14_2: bool = ((s_14_0) == (s_14_1));
        // D s_14_3: not s_14_2
        let s_14_3: bool = !s_14_2;
        // N s_14_4: branch s_14_3 b16 b15
        if s_14_3 {
            return block_16(state, tracer, fn_state);
        } else {
            return block_15(state, tracer, fn_state);
        };
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_15_0: const #0u : u8
        let s_15_0: bool = false;
        // D s_15_1: write-var is_unsigned <= s_15_0
        fn_state.is_unsigned = s_15_0;
        // C s_15_2: const #16s : i
        let s_15_2: i128 = 16;
        // D s_15_3: write-var len <= s_15_2
        fn_state.len = s_15_2;
        // N s_15_4: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_16_0: const #2u : u32
        let s_16_0: u32 = 2;
        // D s_16_1: read-var exttype:u32
        let s_16_1: u32 = fn_state.exttype;
        // D s_16_2: cmp-eq s_16_0 s_16_1
        let s_16_2: bool = ((s_16_0) == (s_16_1));
        // D s_16_3: not s_16_2
        let s_16_3: bool = !s_16_2;
        // N s_16_4: branch s_16_3 b18 b17
        if s_16_3 {
            return block_18(state, tracer, fn_state);
        } else {
            return block_17(state, tracer, fn_state);
        };
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_17_0: const #0u : u8
        let s_17_0: bool = false;
        // D s_17_1: write-var is_unsigned <= s_17_0
        fn_state.is_unsigned = s_17_0;
        // C s_17_2: const #32s : i
        let s_17_2: i128 = 32;
        // D s_17_3: write-var len <= s_17_2
        fn_state.len = s_17_2;
        // N s_17_4: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_18_0: const #3u : u32
        let s_18_0: u32 = 3;
        // D s_18_1: read-var exttype:u32
        let s_18_1: u32 = fn_state.exttype;
        // D s_18_2: cmp-eq s_18_0 s_18_1
        let s_18_2: bool = ((s_18_0) == (s_18_1));
        // D s_18_3: not s_18_2
        let s_18_3: bool = !s_18_2;
        // N s_18_4: branch s_18_3 b20 b19
        if s_18_3 {
            return block_20(state, tracer, fn_state);
        } else {
            return block_19(state, tracer, fn_state);
        };
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_19_0: const #0u : u8
        let s_19_0: bool = false;
        // D s_19_1: write-var is_unsigned <= s_19_0
        fn_state.is_unsigned = s_19_0;
        // C s_19_2: const #64s : i
        let s_19_2: i128 = 64;
        // D s_19_3: write-var len <= s_19_2
        fn_state.len = s_19_2;
        // N s_19_4: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_20_0: const #4u : u32
        let s_20_0: u32 = 4;
        // D s_20_1: read-var exttype:u32
        let s_20_1: u32 = fn_state.exttype;
        // D s_20_2: cmp-eq s_20_0 s_20_1
        let s_20_2: bool = ((s_20_0) == (s_20_1));
        // D s_20_3: not s_20_2
        let s_20_3: bool = !s_20_2;
        // N s_20_4: branch s_20_3 b22 b21
        if s_20_3 {
            return block_22(state, tracer, fn_state);
        } else {
            return block_21(state, tracer, fn_state);
        };
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_21_0: const #1u : u8
        let s_21_0: bool = true;
        // D s_21_1: write-var is_unsigned <= s_21_0
        fn_state.is_unsigned = s_21_0;
        // C s_21_2: const #8s : i
        let s_21_2: i128 = 8;
        // D s_21_3: write-var len <= s_21_2
        fn_state.len = s_21_2;
        // N s_21_4: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_22_0: const #5u : u32
        let s_22_0: u32 = 5;
        // D s_22_1: read-var exttype:u32
        let s_22_1: u32 = fn_state.exttype;
        // D s_22_2: cmp-eq s_22_0 s_22_1
        let s_22_2: bool = ((s_22_0) == (s_22_1));
        // D s_22_3: not s_22_2
        let s_22_3: bool = !s_22_2;
        // N s_22_4: branch s_22_3 b24 b23
        if s_22_3 {
            return block_24(state, tracer, fn_state);
        } else {
            return block_23(state, tracer, fn_state);
        };
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_23_0: const #1u : u8
        let s_23_0: bool = true;
        // D s_23_1: write-var is_unsigned <= s_23_0
        fn_state.is_unsigned = s_23_0;
        // C s_23_2: const #16s : i
        let s_23_2: i128 = 16;
        // D s_23_3: write-var len <= s_23_2
        fn_state.len = s_23_2;
        // N s_23_4: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_24_0: const #6u : u32
        let s_24_0: u32 = 6;
        // D s_24_1: read-var exttype:u32
        let s_24_1: u32 = fn_state.exttype;
        // D s_24_2: cmp-eq s_24_0 s_24_1
        let s_24_2: bool = ((s_24_0) == (s_24_1));
        // D s_24_3: not s_24_2
        let s_24_3: bool = !s_24_2;
        // N s_24_4: branch s_24_3 b26 b25
        if s_24_3 {
            return block_26(state, tracer, fn_state);
        } else {
            return block_25(state, tracer, fn_state);
        };
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_25_0: const #1u : u8
        let s_25_0: bool = true;
        // D s_25_1: write-var is_unsigned <= s_25_0
        fn_state.is_unsigned = s_25_0;
        // C s_25_2: const #32s : i
        let s_25_2: i128 = 32;
        // D s_25_3: write-var len <= s_25_2
        fn_state.len = s_25_2;
        // N s_25_4: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_26_0: const #7u : u32
        let s_26_0: u32 = 7;
        // D s_26_1: read-var exttype:u32
        let s_26_1: u32 = fn_state.exttype;
        // D s_26_2: cmp-eq s_26_0 s_26_1
        let s_26_2: bool = ((s_26_0) == (s_26_1));
        // D s_26_3: not s_26_2
        let s_26_3: bool = !s_26_2;
        // N s_26_4: branch s_26_3 b28 b27
        if s_26_3 {
            return block_28(state, tracer, fn_state);
        } else {
            return block_27(state, tracer, fn_state);
        };
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_27_0: const #1u : u8
        let s_27_0: bool = true;
        // D s_27_1: write-var is_unsigned <= s_27_0
        fn_state.is_unsigned = s_27_0;
        // C s_27_2: const #64s : i
        let s_27_2: i128 = 64;
        // D s_27_3: write-var len <= s_27_2
        fn_state.len = s_27_2;
        // N s_27_4: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // N s_28_0: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_29_0: const #4s : i
        let s_29_0: i128 = 4;
        // D s_29_1: read-var shift:i
        let s_29_1: i128 = fn_state.shift;
        // D s_29_2: cmp-le s_29_1 s_29_0
        let s_29_2: bool = ((s_29_1) <= (s_29_0));
        // D s_29_3: write-var gs#26426 <= s_29_2
        fn_state.gs_26426 = s_29_2;
        // N s_29_4: jump b2
        return block_2(state, tracer, fn_state);
    }
}
