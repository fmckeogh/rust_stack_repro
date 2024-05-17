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
use AArch64_vESBOperation::*;
use FailTransaction::*;
use SendEvent::*;
use Hint_CLRBHB::*;
use TraceSynchronizationBarrier::*;
use Hint_DGH::*;
use AArch64_ChkFeat::*;
use X_read::*;
use ProfilingSynchronizationBarrier::*;
use SendEventLocal::*;
use ConsumptionOfSpeculativeDataBarrier::*;
use HaveTME::*;
use Unreachable::*;
use Hint_WFE::*;
use X_set::*;
use SetBTypeNext::*;
use SynchronizeErrors::*;
use Hint_Yield::*;
use Hint_WFI::*;
use EL2Enabled::*;
use AArch64_ESBOperation::*;
use TakeUnmaskedSErrorInterrupts::*;
use GCSSynchronizationBarrier::*;
use common::*;
pub fn execute_aarch64_instrs_system_hints<T: Tracer>(
    state: &mut State,
    tracer: &T,
    op: u32,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_144844: bool,
        gs_144846: bool,
        gs_144845: bool,
        op: u32,
    }
    let fn_state = FunctionState {
        op,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_0_0: const #1u : u32
        let s_0_0: u32 = 1;
        // D s_0_1: read-var op:u32
        let s_0_1: u32 = fn_state.op;
        // D s_0_2: cmp-eq s_0_0 s_0_1
        let s_0_2: bool = ((s_0_0) == (s_0_1));
        // D s_0_3: not s_0_2
        let s_0_3: bool = !s_0_2;
        // N s_0_4: branch s_0_3 b2 b1
        if s_0_3 {
            return block_2(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_1_0: const #() : ()
        let s_1_0: () = ();
        // S s_1_1: call Hint_Yield(s_1_0)
        let s_1_1: () = Hint_Yield(state, tracer, s_1_0);
        // N s_1_2: return
        return;
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_2_0: const #6u : u32
        let s_2_0: u32 = 6;
        // D s_2_1: read-var op:u32
        let s_2_1: u32 = fn_state.op;
        // D s_2_2: cmp-eq s_2_0 s_2_1
        let s_2_2: bool = ((s_2_0) == (s_2_1));
        // D s_2_3: not s_2_2
        let s_2_3: bool = !s_2_2;
        // N s_2_4: branch s_2_3 b4 b3
        if s_2_3 {
            return block_4(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #() : ()
        let s_3_0: () = ();
        // S s_3_1: call Hint_DGH(s_3_0)
        let s_3_1: () = Hint_DGH(state, tracer, s_3_0);
        // N s_3_2: return
        return;
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_4_0: const #2u : u32
        let s_4_0: u32 = 2;
        // D s_4_1: read-var op:u32
        let s_4_1: u32 = fn_state.op;
        // D s_4_2: cmp-eq s_4_0 s_4_1
        let s_4_2: bool = ((s_4_0) == (s_4_1));
        // D s_4_3: not s_4_2
        let s_4_3: bool = !s_4_2;
        // N s_4_4: branch s_4_3 b6 b5
        if s_4_3 {
            return block_6(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_5_0: const #1s : i
        let s_5_0: i128 = 1;
        // C s_5_1: const #64s : i
        let s_5_1: i128 = 64;
        // C s_5_2: lsl s_5_0 s_5_1
        let s_5_2: i128 = s_5_0 << s_5_1;
        // C s_5_3: const #0u : u32
        let s_5_3: u32 = 0;
        // S s_5_4: call Hint_WFE(s_5_2, s_5_3)
        let s_5_4: () = Hint_WFE(state, tracer, s_5_2, s_5_3);
        // N s_5_5: return
        return;
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #3u : u32
        let s_6_0: u32 = 3;
        // D s_6_1: read-var op:u32
        let s_6_1: u32 = fn_state.op;
        // D s_6_2: cmp-eq s_6_0 s_6_1
        let s_6_2: bool = ((s_6_0) == (s_6_1));
        // D s_6_3: not s_6_2
        let s_6_3: bool = !s_6_2;
        // N s_6_4: branch s_6_3 b8 b7
        if s_6_3 {
            return block_8(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_7_0: const #1s : i
        let s_7_0: i128 = 1;
        // C s_7_1: const #64s : i
        let s_7_1: i128 = 64;
        // C s_7_2: lsl s_7_0 s_7_1
        let s_7_2: i128 = s_7_0 << s_7_1;
        // C s_7_3: const #1u : u32
        let s_7_3: u32 = 1;
        // S s_7_4: call Hint_WFI(s_7_2, s_7_3)
        let s_7_4: () = Hint_WFI(state, tracer, s_7_2, s_7_3);
        // N s_7_5: return
        return;
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_8_0: const #4u : u32
        let s_8_0: u32 = 4;
        // D s_8_1: read-var op:u32
        let s_8_1: u32 = fn_state.op;
        // D s_8_2: cmp-eq s_8_0 s_8_1
        let s_8_2: bool = ((s_8_0) == (s_8_1));
        // D s_8_3: not s_8_2
        let s_8_3: bool = !s_8_2;
        // N s_8_4: branch s_8_3 b10 b9
        if s_8_3 {
            return block_10(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_9_0: const #() : ()
        let s_9_0: () = ();
        // S s_9_1: call SendEvent(s_9_0)
        let s_9_1: () = SendEvent(state, tracer, s_9_0);
        // N s_9_2: return
        return;
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_10_0: const #5u : u32
        let s_10_0: u32 = 5;
        // D s_10_1: read-var op:u32
        let s_10_1: u32 = fn_state.op;
        // D s_10_2: cmp-eq s_10_0 s_10_1
        let s_10_2: bool = ((s_10_0) == (s_10_1));
        // D s_10_3: not s_10_2
        let s_10_3: bool = !s_10_2;
        // N s_10_4: branch s_10_3 b12 b11
        if s_10_3 {
            return block_12(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_11_0: const #() : ()
        let s_11_0: () = ();
        // S s_11_1: call SendEventLocal(s_11_0)
        let s_11_1: () = SendEventLocal(state, tracer, s_11_0);
        // N s_11_2: return
        return;
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_12_0: const #7u : u32
        let s_12_0: u32 = 7;
        // D s_12_1: read-var op:u32
        let s_12_1: u32 = fn_state.op;
        // D s_12_2: cmp-eq s_12_0 s_12_1
        let s_12_2: bool = ((s_12_0) == (s_12_1));
        // D s_12_3: not s_12_2
        let s_12_3: bool = !s_12_2;
        // N s_12_4: branch s_12_3 b29 b13
        if s_12_3 {
            return block_29(state, tracer, fn_state);
        } else {
            return block_13(state, tracer, fn_state);
        };
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_13_0: const #() : ()
        let s_13_0: () = ();
        // S s_13_1: call HaveTME(s_13_0)
        let s_13_1: bool = HaveTME(state, tracer, s_13_0);
        // N s_13_2: branch s_13_1 b28 b14
        if s_13_1 {
            return block_28(state, tracer, fn_state);
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
        // D s_14_1: write-var gs#144844 <= s_14_0
        fn_state.gs_144844 = s_14_0;
        // N s_14_2: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_15_0: read-var gs#144844:u8
        let s_15_0: bool = fn_state.gs_144844;
        // N s_15_1: branch s_15_0 b27 b16
        if s_15_0 {
            return block_27(state, tracer, fn_state);
        } else {
            return block_16(state, tracer, fn_state);
        };
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_16_0: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_17_0: const #() : ()
        let s_17_0: () = ();
        // S s_17_1: call SynchronizeErrors(s_17_0)
        let s_17_1: () = SynchronizeErrors(state, tracer, s_17_0);
        // C s_17_2: const #() : ()
        let s_17_2: () = ();
        // S s_17_3: call AArch64_ESBOperation(s_17_2)
        let s_17_3: () = AArch64_ESBOperation(state, tracer, s_17_2);
        // C s_17_4: const #16975u : u32
        let s_17_4: u32 = 16975;
        // D s_17_5: read-reg s_17_4:u8
        let s_17_5: u8 = {
            let value = state.read_register::<u8>(s_17_4 as isize);
            tracer.read_register(s_17_4 as isize, value);
            value
        };
        // D s_17_6: cast zx s_17_5 -> bv
        let s_17_6: Bits = Bits::new(s_17_5 as u128, 2u16);
        // C s_17_7: const #448u : u32
        let s_17_7: u32 = 448;
        // D s_17_8: read-reg s_17_7:u8
        let s_17_8: u8 = {
            let value = state.read_register::<u8>(s_17_7 as isize);
            tracer.read_register(s_17_7 as isize, value);
            value
        };
        // D s_17_9: cast zx s_17_8 -> bv
        let s_17_9: Bits = Bits::new(s_17_8 as u128, 2u16);
        // D s_17_10: cmp-eq s_17_6 s_17_9
        let s_17_10: bool = ((s_17_6) == (s_17_9));
        // N s_17_11: branch s_17_10 b26 b18
        if s_17_10 {
            return block_26(state, tracer, fn_state);
        } else {
            return block_18(state, tracer, fn_state);
        };
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_18_0: const #16975u : u32
        let s_18_0: u32 = 16975;
        // D s_18_1: read-reg s_18_0:u8
        let s_18_1: u8 = {
            let value = state.read_register::<u8>(s_18_0 as isize);
            tracer.read_register(s_18_0 as isize, value);
            value
        };
        // D s_18_2: cast zx s_18_1 -> bv
        let s_18_2: Bits = Bits::new(s_18_1 as u128, 2u16);
        // C s_18_3: const #440u : u32
        let s_18_3: u32 = 440;
        // D s_18_4: read-reg s_18_3:u8
        let s_18_4: u8 = {
            let value = state.read_register::<u8>(s_18_3 as isize);
            tracer.read_register(s_18_3 as isize, value);
            value
        };
        // D s_18_5: cast zx s_18_4 -> bv
        let s_18_5: Bits = Bits::new(s_18_4 as u128, 2u16);
        // D s_18_6: cmp-eq s_18_2 s_18_5
        let s_18_6: bool = ((s_18_2) == (s_18_5));
        // D s_18_7: write-var gs#144845 <= s_18_6
        fn_state.gs_144845 = s_18_6;
        // N s_18_8: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_19_0: read-var gs#144845:u8
        let s_19_0: bool = fn_state.gs_144845;
        // N s_19_1: branch s_19_0 b25 b20
        if s_19_0 {
            return block_25(state, tracer, fn_state);
        } else {
            return block_20(state, tracer, fn_state);
        };
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_20_0: const #0u : u8
        let s_20_0: bool = false;
        // D s_20_1: write-var gs#144846 <= s_20_0
        fn_state.gs_144846 = s_20_0;
        // N s_20_2: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_21_0: read-var gs#144846:u8
        let s_21_0: bool = fn_state.gs_144846;
        // N s_21_1: branch s_21_0 b24 b22
        if s_21_0 {
            return block_24(state, tracer, fn_state);
        } else {
            return block_22(state, tracer, fn_state);
        };
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_22_0: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_23_0: const #() : ()
        let s_23_0: () = ();
        // S s_23_1: call TakeUnmaskedSErrorInterrupts(s_23_0)
        let s_23_1: () = TakeUnmaskedSErrorInterrupts(state, tracer, s_23_0);
        // N s_23_2: return
        return;
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_24_0: const #() : ()
        let s_24_0: () = ();
        // S s_24_1: call AArch64_vESBOperation(s_24_0)
        let s_24_1: () = AArch64_vESBOperation(state, tracer, s_24_0);
        // N s_24_2: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_25_0: const #() : ()
        let s_25_0: () = ();
        // S s_25_1: call EL2Enabled(s_25_0)
        let s_25_1: bool = EL2Enabled(state, tracer, s_25_0);
        // D s_25_2: write-var gs#144846 <= s_25_1
        fn_state.gs_144846 = s_25_1;
        // N s_25_3: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_26_0: const #1u : u8
        let s_26_0: bool = true;
        // D s_26_1: write-var gs#144845 <= s_26_0
        fn_state.gs_144845 = s_26_0;
        // N s_26_2: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_27_0: const #2u : u32
        let s_27_0: u32 = 2;
        // C s_27_1: const #0u : u8
        let s_27_1: bool = false;
        // S s_27_2: call FailTransaction(s_27_0, s_27_1)
        let s_27_2: () = FailTransaction(state, tracer, s_27_0, s_27_1);
        // N s_27_3: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_28_0: const #100180u : u32
        let s_28_0: u32 = 100180;
        // D s_28_1: read-reg s_28_0:i
        let s_28_1: i128 = {
            let value = state.read_register::<i128>(s_28_0 as isize);
            tracer.read_register(s_28_0 as isize, value);
            value
        };
        // C s_28_2: const #0s : i
        let s_28_2: i128 = 0;
        // D s_28_3: cmp-gt s_28_1 s_28_2
        let s_28_3: bool = ((s_28_1) > (s_28_2));
        // D s_28_4: write-var gs#144844 <= s_28_3
        fn_state.gs_144844 = s_28_3;
        // N s_28_5: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_29_0: const #8u : u32
        let s_29_0: u32 = 8;
        // D s_29_1: read-var op:u32
        let s_29_1: u32 = fn_state.op;
        // D s_29_2: cmp-eq s_29_0 s_29_1
        let s_29_2: bool = ((s_29_0) == (s_29_1));
        // D s_29_3: not s_29_2
        let s_29_3: bool = !s_29_2;
        // N s_29_4: branch s_29_3 b31 b30
        if s_29_3 {
            return block_31(state, tracer, fn_state);
        } else {
            return block_30(state, tracer, fn_state);
        };
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_30_0: const #() : ()
        let s_30_0: () = ();
        // S s_30_1: call ProfilingSynchronizationBarrier(s_30_0)
        let s_30_1: () = ProfilingSynchronizationBarrier(state, tracer, s_30_0);
        // N s_30_2: return
        return;
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_31_0: const #9u : u32
        let s_31_0: u32 = 9;
        // D s_31_1: read-var op:u32
        let s_31_1: u32 = fn_state.op;
        // D s_31_2: cmp-eq s_31_0 s_31_1
        let s_31_2: bool = ((s_31_0) == (s_31_1));
        // D s_31_3: not s_31_2
        let s_31_3: bool = !s_31_2;
        // N s_31_4: branch s_31_3 b33 b32
        if s_31_3 {
            return block_33(state, tracer, fn_state);
        } else {
            return block_32(state, tracer, fn_state);
        };
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_32_0: const #() : ()
        let s_32_0: () = ();
        // S s_32_1: call TraceSynchronizationBarrier(s_32_0)
        let s_32_1: () = TraceSynchronizationBarrier(state, tracer, s_32_0);
        // N s_32_2: return
        return;
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_33_0: const #14u : u32
        let s_33_0: u32 = 14;
        // D s_33_1: read-var op:u32
        let s_33_1: u32 = fn_state.op;
        // D s_33_2: cmp-eq s_33_0 s_33_1
        let s_33_2: bool = ((s_33_0) == (s_33_1));
        // D s_33_3: not s_33_2
        let s_33_3: bool = !s_33_2;
        // N s_33_4: branch s_33_3 b35 b34
        if s_33_3 {
            return block_35(state, tracer, fn_state);
        } else {
            return block_34(state, tracer, fn_state);
        };
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_34_0: const #() : ()
        let s_34_0: () = ();
        // S s_34_1: call GCSSynchronizationBarrier(s_34_0)
        let s_34_1: () = GCSSynchronizationBarrier(state, tracer, s_34_0);
        // N s_34_2: return
        return;
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_35_0: const #15u : u32
        let s_35_0: u32 = 15;
        // D s_35_1: read-var op:u32
        let s_35_1: u32 = fn_state.op;
        // D s_35_2: cmp-eq s_35_0 s_35_1
        let s_35_2: bool = ((s_35_0) == (s_35_1));
        // D s_35_3: not s_35_2
        let s_35_3: bool = !s_35_2;
        // N s_35_4: branch s_35_3 b37 b36
        if s_35_3 {
            return block_37(state, tracer, fn_state);
        } else {
            return block_36(state, tracer, fn_state);
        };
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_36_0: const #64s : i64
        let s_36_0: i64 = 64;
        // C s_36_1: const #64s : i64
        let s_36_1: i64 = 64;
        // C s_36_2: const #16s : i
        let s_36_2: i128 = 16;
        // S s_36_3: call X_read(s_36_2, s_36_1)
        let s_36_3: Bits = X_read(state, tracer, s_36_2, s_36_1);
        // S s_36_4: cast reint s_36_3 -> u64
        let s_36_4: u64 = (s_36_3.value() as u64);
        // S s_36_5: call AArch64_ChkFeat(s_36_4)
        let s_36_5: u64 = AArch64_ChkFeat(state, tracer, s_36_4);
        // C s_36_6: const #16s : i
        let s_36_6: i128 = 16;
        // S s_36_7: cast zx s_36_5 -> bv
        let s_36_7: Bits = Bits::new(s_36_5 as u128, 64u16);
        // S s_36_8: call X_set(s_36_6, s_36_0, s_36_7)
        let s_36_8: () = X_set(state, tracer, s_36_6, s_36_0, s_36_7);
        // N s_36_9: return
        return;
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_37_0: const #16u : u32
        let s_37_0: u32 = 16;
        // D s_37_1: read-var op:u32
        let s_37_1: u32 = fn_state.op;
        // D s_37_2: cmp-eq s_37_0 s_37_1
        let s_37_2: bool = ((s_37_0) == (s_37_1));
        // D s_37_3: not s_37_2
        let s_37_3: bool = !s_37_2;
        // N s_37_4: branch s_37_3 b39 b38
        if s_37_3 {
            return block_39(state, tracer, fn_state);
        } else {
            return block_38(state, tracer, fn_state);
        };
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_38_0: const #() : ()
        let s_38_0: () = ();
        // S s_38_1: call ConsumptionOfSpeculativeDataBarrier(s_38_0)
        let s_38_1: () = ConsumptionOfSpeculativeDataBarrier(state, tracer, s_38_0);
        // N s_38_2: return
        return;
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_39_0: const #13u : u32
        let s_39_0: u32 = 13;
        // D s_39_1: read-var op:u32
        let s_39_1: u32 = fn_state.op;
        // D s_39_2: cmp-eq s_39_0 s_39_1
        let s_39_2: bool = ((s_39_0) == (s_39_1));
        // D s_39_3: not s_39_2
        let s_39_3: bool = !s_39_2;
        // N s_39_4: branch s_39_3 b41 b40
        if s_39_3 {
            return block_41(state, tracer, fn_state);
        } else {
            return block_40(state, tracer, fn_state);
        };
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_40_0: const #() : ()
        let s_40_0: () = ();
        // S s_40_1: call Hint_CLRBHB(s_40_0)
        let s_40_1: () = Hint_CLRBHB(state, tracer, s_40_0);
        // N s_40_2: return
        return;
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_41_0: const #10u : u32
        let s_41_0: u32 = 10;
        // D s_41_1: read-var op:u32
        let s_41_1: u32 = fn_state.op;
        // D s_41_2: cmp-eq s_41_0 s_41_1
        let s_41_2: bool = ((s_41_0) == (s_41_1));
        // D s_41_3: not s_41_2
        let s_41_3: bool = !s_41_2;
        // N s_41_4: branch s_41_3 b43 b42
        if s_41_3 {
            return block_43(state, tracer, fn_state);
        } else {
            return block_42(state, tracer, fn_state);
        };
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_42_0: const #0u : u8
        let s_42_0: u8 = 0;
        // S s_42_1: call SetBTypeNext(s_42_0)
        let s_42_1: () = SetBTypeNext(state, tracer, s_42_0);
        // N s_42_2: return
        return;
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_43_0: const #0u : u32
        let s_43_0: u32 = 0;
        // D s_43_1: read-var op:u32
        let s_43_1: u32 = fn_state.op;
        // D s_43_2: cmp-eq s_43_0 s_43_1
        let s_43_2: bool = ((s_43_0) == (s_43_1));
        // D s_43_3: not s_43_2
        let s_43_3: bool = !s_43_2;
        // N s_43_4: branch s_43_3 b45 b44
        if s_43_3 {
            return block_45(state, tracer, fn_state);
        } else {
            return block_44(state, tracer, fn_state);
        };
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_44_0: return
        return;
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_45_0: const #() : ()
        let s_45_0: () = ();
        // S s_45_1: call Unreachable(s_45_0)
        let s_45_1: () = Unreachable(state, tracer, s_45_0);
        // N s_45_2: return
        return;
    }
}
