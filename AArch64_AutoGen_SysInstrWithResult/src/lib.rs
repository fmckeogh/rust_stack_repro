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
use GCSPOPM_SysInstrWithResult_785f34ad5cd557fe::*;
use GCSSS2_SysInstrWithResult_f9ffdfaa333b34bf::*;
use AArch64_UnallocatedSysRegAccess::*;
use common::*;
pub fn AArch64_AutoGen_SysInstrWithResult<T: Tracer>(
    state: &mut State,
    tracer: &T,
    el: u8,
    op0: u8,
    op1: u8,
    CRn: u8,
    op2: u8,
    CRm: u8,
    t: i128,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_104434: bool,
        gs_104428: bool,
        gs_104430: bool,
        gs_104432: bool,
        gs_104433: bool,
        gs_104431: bool,
        gs_104429: bool,
        gs_104427: bool,
        el: u8,
        op0: u8,
        op1: u8,
        CRn: u8,
        op2: u8,
        CRm: u8,
        t: i128,
    }
    let fn_state = FunctionState {
        el,
        op0,
        op1,
        CRn,
        op2,
        CRm,
        t,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_0_0: read-var CRm:u8
        let s_0_0: u8 = fn_state.CRm;
        // D s_0_1: cast zx s_0_0 -> bv
        let s_0_1: Bits = Bits::new(s_0_0 as u128, 4u16);
        // C s_0_2: const #7u : u8
        let s_0_2: u8 = 7;
        // C s_0_3: cast zx s_0_2 -> bv
        let s_0_3: Bits = Bits::new(s_0_2 as u128, 4u16);
        // D s_0_4: cmp-eq s_0_1 s_0_3
        let s_0_4: bool = ((s_0_1) == (s_0_3));
        // N s_0_5: branch s_0_4 b28 b1
        if s_0_4 {
            return block_28(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_1_0: const #0u : u8
        let s_1_0: bool = false;
        // D s_1_1: write-var gs#104427 <= s_1_0
        fn_state.gs_104427 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var gs#104427:u8
        let s_2_0: bool = fn_state.gs_104427;
        // N s_2_1: branch s_2_0 b27 b3
        if s_2_0 {
            return block_27(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #0u : u8
        let s_3_0: bool = false;
        // D s_3_1: write-var gs#104428 <= s_3_0
        fn_state.gs_104428 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var gs#104428:u8
        let s_4_0: bool = fn_state.gs_104428;
        // N s_4_1: branch s_4_0 b26 b5
        if s_4_0 {
            return block_26(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_5_0: const #0u : u8
        let s_5_0: bool = false;
        // D s_5_1: write-var gs#104429 <= s_5_0
        fn_state.gs_104429 = s_5_0;
        // N s_5_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var gs#104429:u8
        let s_6_0: bool = fn_state.gs_104429;
        // N s_6_1: branch s_6_0 b25 b7
        if s_6_0 {
            return block_25(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_7_0: const #0u : u8
        let s_7_0: bool = false;
        // D s_7_1: write-var gs#104430 <= s_7_0
        fn_state.gs_104430 = s_7_0;
        // N s_7_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var gs#104430:u8
        let s_8_0: bool = fn_state.gs_104430;
        // N s_8_1: branch s_8_0 b24 b9
        if s_8_0 {
            return block_24(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var CRm:u8
        let s_9_0: u8 = fn_state.CRm;
        // D s_9_1: cast zx s_9_0 -> bv
        let s_9_1: Bits = Bits::new(s_9_0 as u128, 4u16);
        // C s_9_2: const #7u : u8
        let s_9_2: u8 = 7;
        // C s_9_3: cast zx s_9_2 -> bv
        let s_9_3: Bits = Bits::new(s_9_2 as u128, 4u16);
        // D s_9_4: cmp-eq s_9_1 s_9_3
        let s_9_4: bool = ((s_9_1) == (s_9_3));
        // N s_9_5: branch s_9_4 b23 b10
        if s_9_4 {
            return block_23(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_10_0: const #0u : u8
        let s_10_0: bool = false;
        // D s_10_1: write-var gs#104431 <= s_10_0
        fn_state.gs_104431 = s_10_0;
        // N s_10_2: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var gs#104431:u8
        let s_11_0: bool = fn_state.gs_104431;
        // N s_11_1: branch s_11_0 b22 b12
        if s_11_0 {
            return block_22(state, tracer, fn_state);
        } else {
            return block_12(state, tracer, fn_state);
        };
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_12_0: const #0u : u8
        let s_12_0: bool = false;
        // D s_12_1: write-var gs#104432 <= s_12_0
        fn_state.gs_104432 = s_12_0;
        // N s_12_2: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var gs#104432:u8
        let s_13_0: bool = fn_state.gs_104432;
        // N s_13_1: branch s_13_0 b21 b14
        if s_13_0 {
            return block_21(state, tracer, fn_state);
        } else {
            return block_14(state, tracer, fn_state);
        };
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_14_0: const #0u : u8
        let s_14_0: bool = false;
        // D s_14_1: write-var gs#104433 <= s_14_0
        fn_state.gs_104433 = s_14_0;
        // N s_14_2: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_15_0: read-var gs#104433:u8
        let s_15_0: bool = fn_state.gs_104433;
        // N s_15_1: branch s_15_0 b20 b16
        if s_15_0 {
            return block_20(state, tracer, fn_state);
        } else {
            return block_16(state, tracer, fn_state);
        };
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_16_0: const #0u : u8
        let s_16_0: bool = false;
        // D s_16_1: write-var gs#104434 <= s_16_0
        fn_state.gs_104434 = s_16_0;
        // N s_16_2: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_17_0: read-var gs#104434:u8
        let s_17_0: bool = fn_state.gs_104434;
        // N s_17_1: branch s_17_0 b19 b18
        if s_17_0 {
            return block_19(state, tracer, fn_state);
        } else {
            return block_18(state, tracer, fn_state);
        };
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_18_0: read-var op0:u8
        let s_18_0: u8 = fn_state.op0;
        // D s_18_1: read-var op1:u8
        let s_18_1: u8 = fn_state.op1;
        // D s_18_2: read-var CRn:u8
        let s_18_2: u8 = fn_state.CRn;
        // D s_18_3: read-var op2:u8
        let s_18_3: u8 = fn_state.op2;
        // D s_18_4: read-var CRm:u8
        let s_18_4: u8 = fn_state.CRm;
        // C s_18_5: const #0u : u8
        let s_18_5: bool = false;
        // D s_18_6: read-var t:i
        let s_18_6: i128 = fn_state.t;
        // D s_18_7: call AArch64_UnallocatedSysRegAccess(s_18_0, s_18_1, s_18_2, s_18_3, s_18_4, s_18_5, s_18_6)
        let s_18_7: () = AArch64_UnallocatedSysRegAccess(
            state,
            tracer,
            s_18_0,
            s_18_1,
            s_18_2,
            s_18_3,
            s_18_4,
            s_18_5,
            s_18_6,
        );
        // N s_18_8: return
        return;
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_19_0: read-var el:u8
        let s_19_0: u8 = fn_state.el;
        // D s_19_1: read-var op0:u8
        let s_19_1: u8 = fn_state.op0;
        // D s_19_2: read-var op1:u8
        let s_19_2: u8 = fn_state.op1;
        // D s_19_3: read-var CRn:u8
        let s_19_3: u8 = fn_state.CRn;
        // D s_19_4: read-var op2:u8
        let s_19_4: u8 = fn_state.op2;
        // D s_19_5: read-var CRm:u8
        let s_19_5: u8 = fn_state.CRm;
        // D s_19_6: read-var t:i
        let s_19_6: i128 = fn_state.t;
        // D s_19_7: call GCSPOPM_SysInstrWithResult_785f34ad5cd557fe(s_19_0, s_19_1, s_19_2, s_19_3, s_19_4, s_19_5, s_19_6)
        let s_19_7: () = GCSPOPM_SysInstrWithResult_785f34ad5cd557fe(
            state,
            tracer,
            s_19_0,
            s_19_1,
            s_19_2,
            s_19_3,
            s_19_4,
            s_19_5,
            s_19_6,
        );
        // N s_19_8: return
        return;
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_20_0: read-var op2:u8
        let s_20_0: u8 = fn_state.op2;
        // D s_20_1: cast zx s_20_0 -> bv
        let s_20_1: Bits = Bits::new(s_20_0 as u128, 3u16);
        // C s_20_2: const #1u : u8
        let s_20_2: u8 = 1;
        // C s_20_3: cast zx s_20_2 -> bv
        let s_20_3: Bits = Bits::new(s_20_2 as u128, 3u16);
        // D s_20_4: cmp-eq s_20_1 s_20_3
        let s_20_4: bool = ((s_20_1) == (s_20_3));
        // D s_20_5: write-var gs#104434 <= s_20_4
        fn_state.gs_104434 = s_20_4;
        // N s_20_6: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_21_0: read-var op1:u8
        let s_21_0: u8 = fn_state.op1;
        // D s_21_1: cast zx s_21_0 -> bv
        let s_21_1: Bits = Bits::new(s_21_0 as u128, 3u16);
        // C s_21_2: const #3u : u8
        let s_21_2: u8 = 3;
        // C s_21_3: cast zx s_21_2 -> bv
        let s_21_3: Bits = Bits::new(s_21_2 as u128, 3u16);
        // D s_21_4: cmp-eq s_21_1 s_21_3
        let s_21_4: bool = ((s_21_1) == (s_21_3));
        // D s_21_5: write-var gs#104433 <= s_21_4
        fn_state.gs_104433 = s_21_4;
        // N s_21_6: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_22_0: read-var op0:u8
        let s_22_0: u8 = fn_state.op0;
        // D s_22_1: cast zx s_22_0 -> bv
        let s_22_1: Bits = Bits::new(s_22_0 as u128, 2u16);
        // C s_22_2: const #1u : u8
        let s_22_2: u8 = 1;
        // C s_22_3: cast zx s_22_2 -> bv
        let s_22_3: Bits = Bits::new(s_22_2 as u128, 2u16);
        // D s_22_4: cmp-eq s_22_1 s_22_3
        let s_22_4: bool = ((s_22_1) == (s_22_3));
        // D s_22_5: write-var gs#104432 <= s_22_4
        fn_state.gs_104432 = s_22_4;
        // N s_22_6: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_23_0: read-var CRn:u8
        let s_23_0: u8 = fn_state.CRn;
        // D s_23_1: cast zx s_23_0 -> bv
        let s_23_1: Bits = Bits::new(s_23_0 as u128, 4u16);
        // C s_23_2: const #7u : u8
        let s_23_2: u8 = 7;
        // C s_23_3: cast zx s_23_2 -> bv
        let s_23_3: Bits = Bits::new(s_23_2 as u128, 4u16);
        // D s_23_4: cmp-eq s_23_1 s_23_3
        let s_23_4: bool = ((s_23_1) == (s_23_3));
        // D s_23_5: write-var gs#104431 <= s_23_4
        fn_state.gs_104431 = s_23_4;
        // N s_23_6: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_24_0: read-var el:u8
        let s_24_0: u8 = fn_state.el;
        // D s_24_1: read-var op0:u8
        let s_24_1: u8 = fn_state.op0;
        // D s_24_2: read-var op1:u8
        let s_24_2: u8 = fn_state.op1;
        // D s_24_3: read-var CRn:u8
        let s_24_3: u8 = fn_state.CRn;
        // D s_24_4: read-var op2:u8
        let s_24_4: u8 = fn_state.op2;
        // D s_24_5: read-var CRm:u8
        let s_24_5: u8 = fn_state.CRm;
        // D s_24_6: read-var t:i
        let s_24_6: i128 = fn_state.t;
        // D s_24_7: call GCSSS2_SysInstrWithResult_f9ffdfaa333b34bf(s_24_0, s_24_1, s_24_2, s_24_3, s_24_4, s_24_5, s_24_6)
        let s_24_7: () = GCSSS2_SysInstrWithResult_f9ffdfaa333b34bf(
            state,
            tracer,
            s_24_0,
            s_24_1,
            s_24_2,
            s_24_3,
            s_24_4,
            s_24_5,
            s_24_6,
        );
        // N s_24_8: return
        return;
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_25_0: read-var op2:u8
        let s_25_0: u8 = fn_state.op2;
        // D s_25_1: cast zx s_25_0 -> bv
        let s_25_1: Bits = Bits::new(s_25_0 as u128, 3u16);
        // C s_25_2: const #3u : u8
        let s_25_2: u8 = 3;
        // C s_25_3: cast zx s_25_2 -> bv
        let s_25_3: Bits = Bits::new(s_25_2 as u128, 3u16);
        // D s_25_4: cmp-eq s_25_1 s_25_3
        let s_25_4: bool = ((s_25_1) == (s_25_3));
        // D s_25_5: write-var gs#104430 <= s_25_4
        fn_state.gs_104430 = s_25_4;
        // N s_25_6: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_26_0: read-var op1:u8
        let s_26_0: u8 = fn_state.op1;
        // D s_26_1: cast zx s_26_0 -> bv
        let s_26_1: Bits = Bits::new(s_26_0 as u128, 3u16);
        // C s_26_2: const #3u : u8
        let s_26_2: u8 = 3;
        // C s_26_3: cast zx s_26_2 -> bv
        let s_26_3: Bits = Bits::new(s_26_2 as u128, 3u16);
        // D s_26_4: cmp-eq s_26_1 s_26_3
        let s_26_4: bool = ((s_26_1) == (s_26_3));
        // D s_26_5: write-var gs#104429 <= s_26_4
        fn_state.gs_104429 = s_26_4;
        // N s_26_6: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_27_0: read-var op0:u8
        let s_27_0: u8 = fn_state.op0;
        // D s_27_1: cast zx s_27_0 -> bv
        let s_27_1: Bits = Bits::new(s_27_0 as u128, 2u16);
        // C s_27_2: const #1u : u8
        let s_27_2: u8 = 1;
        // C s_27_3: cast zx s_27_2 -> bv
        let s_27_3: Bits = Bits::new(s_27_2 as u128, 2u16);
        // D s_27_4: cmp-eq s_27_1 s_27_3
        let s_27_4: bool = ((s_27_1) == (s_27_3));
        // D s_27_5: write-var gs#104428 <= s_27_4
        fn_state.gs_104428 = s_27_4;
        // N s_27_6: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_28_0: read-var CRn:u8
        let s_28_0: u8 = fn_state.CRn;
        // D s_28_1: cast zx s_28_0 -> bv
        let s_28_1: Bits = Bits::new(s_28_0 as u128, 4u16);
        // C s_28_2: const #7u : u8
        let s_28_2: u8 = 7;
        // C s_28_3: cast zx s_28_2 -> bv
        let s_28_3: Bits = Bits::new(s_28_2 as u128, 4u16);
        // D s_28_4: cmp-eq s_28_1 s_28_3
        let s_28_4: bool = ((s_28_1) == (s_28_3));
        // D s_28_5: write-var gs#104427 <= s_28_4
        fn_state.gs_104427 = s_28_4;
        // N s_28_6: jump b2
        return block_2(state, tracer, fn_state);
    }
}
