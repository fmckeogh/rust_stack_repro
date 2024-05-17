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
use IsDebugException::*;
use HCR2_read::*;
use DBGDSCRext_read::*;
use DBGDSCRext_write::*;
use u_update_DBGDSCRext_Type_MOE::*;
use HCR_read::*;
use HDCR_read::*;
use u_get_HDCR_Type_TDE::*;
use AArch32_EnterMonitorMode::*;
use AArch32_ReportDataAbort::*;
use IsExternalAbort__1::*;
use u_get_HCR_Type_TGE::*;
use AArch32_EnterHypMode::*;
use ThisInstrAddr::*;
use IsSecondStage::*;
use u_get_HCR2_Type_TEA::*;
use EffectiveEA::*;
use EL2Enabled::*;
use HaveRASExt::*;
use AArch32_AbortSyndrome::*;
use AArch32_EnterMode::*;
use common::*;
pub fn AArch32_TakeDataAbortException<T: Tracer>(
    state: &mut State,
    tracer: &T,
    vaddress: u32,
    fault: ProductType1d757adad216cdef,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_9528: bool,
        route_to_monitor: bool,
        route_to_hyp: bool,
        gs_9537: bool,
        exceptshadow_126: ProductTypeb7f99f96751e17c4,
        gs_9533: bool,
        gs_9529: bool,
        gs_9531: bool,
        preferred_exception_return: u32,
        gs_9532: bool,
        gs_9538: bool,
        gs_9535: bool,
        gs_9536: bool,
        vect_offset: i64,
        gs_9534: bool,
        gs_9545: bool,
        gs_9530: bool,
        vaddress: u32,
        fault: ProductType1d757adad216cdef,
    }
    let fn_state = FunctionState {
        vaddress,
        fault,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_0_0: const #424u : u32
        let s_0_0: u32 = 424;
        // D s_0_1: read-reg s_0_0:u8
        let s_0_1: u8 = {
            let value = state.read_register::<u8>(s_0_0 as isize);
            tracer.read_register(s_0_0 as isize, value);
            value
        };
        // C s_0_2: const #2u : u8
        let s_0_2: u8 = 2;
        // D s_0_3: cmp-lt s_0_1 s_0_2
        let s_0_3: bool = ((s_0_1) < (s_0_2));
        // N s_0_4: branch s_0_3 b45 b1
        if s_0_3 {
            return block_45(state, tracer, fn_state);
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
        // D s_1_1: write-var gs#9528 <= s_1_0
        fn_state.gs_9528 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var gs#9528:u8
        let s_2_0: bool = fn_state.gs_9528;
        // N s_2_1: branch s_2_0 b44 b3
        if s_2_0 {
            return block_44(state, tracer, fn_state);
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
        // D s_3_1: write-var gs#9529 <= s_3_0
        fn_state.gs_9529 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var gs#9529:u8
        let s_4_0: bool = fn_state.gs_9529;
        // D s_4_1: write-var route_to_monitor <= s_4_0
        fn_state.route_to_monitor = s_4_0;
        // C s_4_2: const #() : ()
        let s_4_2: () = ();
        // S s_4_3: call EL2Enabled(s_4_2)
        let s_4_3: bool = EL2Enabled(state, tracer, s_4_2);
        // N s_4_4: branch s_4_3 b40 b5
        if s_4_3 {
            return block_40(state, tracer, fn_state);
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
        // D s_5_1: write-var gs#9531 <= s_5_0
        fn_state.gs_9531 = s_5_0;
        // N s_5_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var gs#9531:u8
        let s_6_0: bool = fn_state.gs_9531;
        // N s_6_1: branch s_6_0 b21 b7
        if s_6_0 {
            return block_21(state, tracer, fn_state);
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
        // D s_7_1: write-var gs#9538 <= s_7_0
        fn_state.gs_9538 = s_7_0;
        // N s_7_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var gs#9538:u8
        let s_8_0: bool = fn_state.gs_9538;
        // D s_8_1: write-var route_to_hyp <= s_8_0
        fn_state.route_to_hyp = s_8_0;
        // C s_8_2: const #32s : i64
        let s_8_2: i64 = 32;
        // C s_8_3: cast zx s_8_2 -> i
        let s_8_3: i128 = (i128::try_from(s_8_2).unwrap());
        // S s_8_4: call ThisInstrAddr(s_8_3)
        let s_8_4: Bits = ThisInstrAddr(state, tracer, s_8_3);
        // S s_8_5: cast reint s_8_4 -> u32
        let s_8_5: u32 = (s_8_4.value() as u32);
        // D s_8_6: write-var preferred_exception_return <= s_8_5
        fn_state.preferred_exception_return = s_8_5;
        // C s_8_7: const #16u : u8
        let s_8_7: u8 = 16;
        // C s_8_8: cast zx s_8_7 -> bv
        let s_8_8: Bits = Bits::new(s_8_7 as u128, 8u16);
        // C s_8_9: cast zx s_8_8 -> i
        let s_8_9: i128 = (s_8_8.value() as i128);
        // C s_8_10: cast reint s_8_9 -> i64
        let s_8_10: i64 = (s_8_9 as i64);
        // D s_8_11: write-var vect_offset <= s_8_10
        fn_state.vect_offset = s_8_10;
        // D s_8_12: read-var fault:struct
        let s_8_12: ProductType1d757adad216cdef = fn_state.fault;
        // D s_8_13: call IsDebugException(s_8_12)
        let s_8_13: bool = IsDebugException(state, tracer, s_8_12);
        // N s_8_14: branch s_8_13 b20 b9
        if s_8_13 {
            return block_20(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_9_0: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var route_to_monitor:u8
        let s_10_0: bool = fn_state.route_to_monitor;
        // N s_10_1: branch s_10_0 b19 b11
        if s_10_0 {
            return block_19(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_11_0: const #16975u : u32
        let s_11_0: u32 = 16975;
        // D s_11_1: read-reg s_11_0:u8
        let s_11_1: u8 = {
            let value = state.read_register::<u8>(s_11_0 as isize);
            tracer.read_register(s_11_0 as isize, value);
            value
        };
        // D s_11_2: cast zx s_11_1 -> bv
        let s_11_2: Bits = Bits::new(s_11_1 as u128, 2u16);
        // C s_11_3: const #432u : u32
        let s_11_3: u32 = 432;
        // D s_11_4: read-reg s_11_3:u8
        let s_11_4: u8 = {
            let value = state.read_register::<u8>(s_11_3 as isize);
            tracer.read_register(s_11_3 as isize, value);
            value
        };
        // D s_11_5: cast zx s_11_4 -> bv
        let s_11_5: Bits = Bits::new(s_11_4 as u128, 2u16);
        // D s_11_6: cmp-eq s_11_2 s_11_5
        let s_11_6: bool = ((s_11_2) == (s_11_5));
        // N s_11_7: branch s_11_6 b18 b12
        if s_11_6 {
            return block_18(state, tracer, fn_state);
        } else {
            return block_12(state, tracer, fn_state);
        };
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var route_to_hyp:u8
        let s_12_0: bool = fn_state.route_to_hyp;
        // D s_12_1: write-var gs#9545 <= s_12_0
        fn_state.gs_9545 = s_12_0;
        // N s_12_2: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var gs#9545:u8
        let s_13_0: bool = fn_state.gs_9545;
        // N s_13_1: branch s_13_0 b15 b14
        if s_13_0 {
            return block_15(state, tracer, fn_state);
        } else {
            return block_14(state, tracer, fn_state);
        };
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var route_to_monitor:u8
        let s_14_0: bool = fn_state.route_to_monitor;
        // D s_14_1: read-var fault:struct
        let s_14_1: ProductType1d757adad216cdef = fn_state.fault;
        // D s_14_2: read-var vaddress:u32
        let s_14_2: u32 = fn_state.vaddress;
        // D s_14_3: call AArch32_ReportDataAbort(s_14_0, s_14_1, s_14_2)
        let s_14_3: () = AArch32_ReportDataAbort(state, tracer, s_14_0, s_14_1, s_14_2);
        // C s_14_4: const #8s : i64
        let s_14_4: i64 = 8;
        // C s_14_5: cast zx s_14_4 -> i
        let s_14_5: i128 = (i128::try_from(s_14_4).unwrap());
        // D s_14_6: read-var vect_offset:i64
        let s_14_6: i64 = fn_state.vect_offset;
        // D s_14_7: cast zx s_14_6 -> i
        let s_14_7: i128 = (i128::try_from(s_14_6).unwrap());
        // C s_14_8: const #392u : u32
        let s_14_8: u32 = 392;
        // D s_14_9: read-reg s_14_8:u8
        let s_14_9: u8 = {
            let value = state.read_register::<u8>(s_14_8 as isize);
            tracer.read_register(s_14_8 as isize, value);
            value
        };
        // D s_14_10: read-var preferred_exception_return:u32
        let s_14_10: u32 = fn_state.preferred_exception_return;
        // D s_14_11: call AArch32_EnterMode(s_14_9, s_14_10, s_14_5, s_14_7)
        let s_14_11: () = AArch32_EnterMode(
            state,
            tracer,
            s_14_9,
            s_14_10,
            s_14_5,
            s_14_7,
        );
        // N s_14_12: return
        return;
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_15_0: const #19u : u32
        let s_15_0: u32 = 19;
        // D s_15_1: read-var fault:struct
        let s_15_1: ProductType1d757adad216cdef = fn_state.fault;
        // D s_15_2: read-var vaddress:u32
        let s_15_2: u32 = fn_state.vaddress;
        // C s_15_3: const #432u : u32
        let s_15_3: u32 = 432;
        // D s_15_4: read-reg s_15_3:u8
        let s_15_4: u8 = {
            let value = state.read_register::<u8>(s_15_3 as isize);
            tracer.read_register(s_15_3 as isize, value);
            value
        };
        // D s_15_5: call AArch32_AbortSyndrome(s_15_0, s_15_1, s_15_2, s_15_4)
        let s_15_5: ProductTypeb7f99f96751e17c4 = AArch32_AbortSyndrome(
            state,
            tracer,
            s_15_0,
            s_15_1,
            s_15_2,
            s_15_4,
        );
        // D s_15_6: write-var exceptshadow#126 <= s_15_5
        fn_state.exceptshadow_126 = s_15_5;
        // C s_15_7: const #16975u : u32
        let s_15_7: u32 = 16975;
        // D s_15_8: read-reg s_15_7:u8
        let s_15_8: u8 = {
            let value = state.read_register::<u8>(s_15_7 as isize);
            tracer.read_register(s_15_7 as isize, value);
            value
        };
        // D s_15_9: cast zx s_15_8 -> bv
        let s_15_9: Bits = Bits::new(s_15_8 as u128, 2u16);
        // C s_15_10: const #432u : u32
        let s_15_10: u32 = 432;
        // D s_15_11: read-reg s_15_10:u8
        let s_15_11: u8 = {
            let value = state.read_register::<u8>(s_15_10 as isize);
            tracer.read_register(s_15_10 as isize, value);
            value
        };
        // D s_15_12: cast zx s_15_11 -> bv
        let s_15_12: Bits = Bits::new(s_15_11 as u128, 2u16);
        // D s_15_13: cmp-eq s_15_9 s_15_12
        let s_15_13: bool = ((s_15_9) == (s_15_12));
        // N s_15_14: branch s_15_13 b17 b16
        if s_15_13 {
            return block_17(state, tracer, fn_state);
        } else {
            return block_16(state, tracer, fn_state);
        };
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_16_0: const #20u : u8
        let s_16_0: u8 = 20;
        // C s_16_1: cast zx s_16_0 -> bv
        let s_16_1: Bits = Bits::new(s_16_0 as u128, 8u16);
        // C s_16_2: cast zx s_16_1 -> i
        let s_16_2: i128 = (s_16_1.value() as i128);
        // C s_16_3: cast reint s_16_2 -> i64
        let s_16_3: i64 = (s_16_2 as i64);
        // C s_16_4: cast zx s_16_3 -> i
        let s_16_4: i128 = (i128::try_from(s_16_3).unwrap());
        // D s_16_5: read-var exceptshadow#126:struct
        let s_16_5: ProductTypeb7f99f96751e17c4 = fn_state.exceptshadow_126;
        // D s_16_6: read-var preferred_exception_return:u32
        let s_16_6: u32 = fn_state.preferred_exception_return;
        // D s_16_7: call AArch32_EnterHypMode(s_16_5, s_16_6, s_16_4)
        let s_16_7: () = AArch32_EnterHypMode(state, tracer, s_16_5, s_16_6, s_16_4);
        // N s_16_8: return
        return;
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_17_0: read-var vect_offset:i64
        let s_17_0: i64 = fn_state.vect_offset;
        // D s_17_1: cast zx s_17_0 -> i
        let s_17_1: i128 = (i128::try_from(s_17_0).unwrap());
        // D s_17_2: read-var exceptshadow#126:struct
        let s_17_2: ProductTypeb7f99f96751e17c4 = fn_state.exceptshadow_126;
        // D s_17_3: read-var preferred_exception_return:u32
        let s_17_3: u32 = fn_state.preferred_exception_return;
        // D s_17_4: call AArch32_EnterHypMode(s_17_2, s_17_3, s_17_1)
        let s_17_4: () = AArch32_EnterHypMode(state, tracer, s_17_2, s_17_3, s_17_1);
        // N s_17_5: return
        return;
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_18_0: const #1u : u8
        let s_18_0: bool = true;
        // D s_18_1: write-var gs#9545 <= s_18_0
        fn_state.gs_9545 = s_18_0;
        // N s_18_2: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_19_0: read-var route_to_monitor:u8
        let s_19_0: bool = fn_state.route_to_monitor;
        // D s_19_1: read-var fault:struct
        let s_19_1: ProductType1d757adad216cdef = fn_state.fault;
        // D s_19_2: read-var vaddress:u32
        let s_19_2: u32 = fn_state.vaddress;
        // D s_19_3: call AArch32_ReportDataAbort(s_19_0, s_19_1, s_19_2)
        let s_19_3: () = AArch32_ReportDataAbort(state, tracer, s_19_0, s_19_1, s_19_2);
        // C s_19_4: const #8s : i64
        let s_19_4: i64 = 8;
        // C s_19_5: cast zx s_19_4 -> i
        let s_19_5: i128 = (i128::try_from(s_19_4).unwrap());
        // D s_19_6: read-var vect_offset:i64
        let s_19_6: i64 = fn_state.vect_offset;
        // D s_19_7: cast zx s_19_6 -> i
        let s_19_7: i128 = (i128::try_from(s_19_6).unwrap());
        // D s_19_8: read-var preferred_exception_return:u32
        let s_19_8: u32 = fn_state.preferred_exception_return;
        // D s_19_9: call AArch32_EnterMonitorMode(s_19_8, s_19_5, s_19_7)
        let s_19_9: () = AArch32_EnterMonitorMode(state, tracer, s_19_8, s_19_5, s_19_7);
        // N s_19_10: return
        return;
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_20_0: const #() : ()
        let s_20_0: () = ();
        // S s_20_1: call DBGDSCRext_read(s_20_0)
        let s_20_1: ProductType700c18a878c5601b = DBGDSCRext_read(state, tracer, s_20_0);
        // D s_20_2: read-var fault.2:struct
        let s_20_2: u8 = fn_state.fault._2;
        // D s_20_3: call _update_DBGDSCRext_Type_MOE(s_20_1, s_20_2)
        let s_20_3: ProductType700c18a878c5601b = u_update_DBGDSCRext_Type_MOE(
            state,
            tracer,
            s_20_1,
            s_20_2,
        );
        // D s_20_4: call DBGDSCRext_write(s_20_3)
        let s_20_4: () = DBGDSCRext_write(state, tracer, s_20_3);
        // N s_20_5: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_21_0: const #() : ()
        let s_21_0: () = ();
        // S s_21_1: call HCR_read(s_21_0)
        let s_21_1: ProductType700c18a878c5601b = HCR_read(state, tracer, s_21_0);
        // S s_21_2: call _get_HCR_Type_TGE(s_21_1)
        let s_21_2: bool = u_get_HCR_Type_TGE(state, tracer, s_21_1);
        // S s_21_3: cast zx s_21_2 -> bv
        let s_21_3: Bits = Bits::new(s_21_2 as u128, 1u16);
        // C s_21_4: const #1u : u8
        let s_21_4: bool = true;
        // C s_21_5: cast zx s_21_4 -> bv
        let s_21_5: Bits = Bits::new(s_21_4 as u128, 1u16);
        // S s_21_6: cmp-eq s_21_3 s_21_5
        let s_21_6: bool = ((s_21_3) == (s_21_5));
        // N s_21_7: branch s_21_6 b39 b22
        if s_21_6 {
            return block_39(state, tracer, fn_state);
        } else {
            return block_22(state, tracer, fn_state);
        };
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_22_0: const #() : ()
        let s_22_0: () = ();
        // S s_22_1: call HaveRASExt(s_22_0)
        let s_22_1: bool = HaveRASExt(state, tracer, s_22_0);
        // N s_22_2: branch s_22_1 b38 b23
        if s_22_1 {
            return block_38(state, tracer, fn_state);
        } else {
            return block_23(state, tracer, fn_state);
        };
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_23_0: const #0u : u8
        let s_23_0: bool = false;
        // D s_23_1: write-var gs#9532 <= s_23_0
        fn_state.gs_9532 = s_23_0;
        // N s_23_2: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_24_0: read-var gs#9532:u8
        let s_24_0: bool = fn_state.gs_9532;
        // N s_24_1: branch s_24_0 b37 b25
        if s_24_0 {
            return block_37(state, tracer, fn_state);
        } else {
            return block_25(state, tracer, fn_state);
        };
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_25_0: const #0u : u8
        let s_25_0: bool = false;
        // D s_25_1: write-var gs#9533 <= s_25_0
        fn_state.gs_9533 = s_25_0;
        // N s_25_2: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_26_0: read-var gs#9533:u8
        let s_26_0: bool = fn_state.gs_9533;
        // D s_26_1: write-var gs#9534 <= s_26_0
        fn_state.gs_9534 = s_26_0;
        // N s_26_2: jump b27
        return block_27(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_27_0: read-var gs#9534:u8
        let s_27_0: bool = fn_state.gs_9534;
        // N s_27_1: branch s_27_0 b36 b28
        if s_27_0 {
            return block_36(state, tracer, fn_state);
        } else {
            return block_28(state, tracer, fn_state);
        };
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_28_0: read-var fault:struct
        let s_28_0: ProductType1d757adad216cdef = fn_state.fault;
        // D s_28_1: call IsDebugException(s_28_0)
        let s_28_1: bool = IsDebugException(state, tracer, s_28_0);
        // N s_28_2: branch s_28_1 b35 b29
        if s_28_1 {
            return block_35(state, tracer, fn_state);
        } else {
            return block_29(state, tracer, fn_state);
        };
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_29_0: const #0u : u8
        let s_29_0: bool = false;
        // D s_29_1: write-var gs#9535 <= s_29_0
        fn_state.gs_9535 = s_29_0;
        // N s_29_2: jump b30
        return block_30(state, tracer, fn_state);
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_30_0: read-var gs#9535:u8
        let s_30_0: bool = fn_state.gs_9535;
        // D s_30_1: write-var gs#9536 <= s_30_0
        fn_state.gs_9536 = s_30_0;
        // N s_30_2: jump b31
        return block_31(state, tracer, fn_state);
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_31_0: read-var gs#9536:u8
        let s_31_0: bool = fn_state.gs_9536;
        // N s_31_1: branch s_31_0 b34 b32
        if s_31_0 {
            return block_34(state, tracer, fn_state);
        } else {
            return block_32(state, tracer, fn_state);
        };
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_32_0: read-var fault:struct
        let s_32_0: ProductType1d757adad216cdef = fn_state.fault;
        // D s_32_1: call IsSecondStage(s_32_0)
        let s_32_1: bool = IsSecondStage(state, tracer, s_32_0);
        // D s_32_2: write-var gs#9537 <= s_32_1
        fn_state.gs_9537 = s_32_1;
        // N s_32_3: jump b33
        return block_33(state, tracer, fn_state);
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_33_0: read-var gs#9537:u8
        let s_33_0: bool = fn_state.gs_9537;
        // D s_33_1: write-var gs#9538 <= s_33_0
        fn_state.gs_9538 = s_33_0;
        // N s_33_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_34_0: const #1u : u8
        let s_34_0: bool = true;
        // D s_34_1: write-var gs#9537 <= s_34_0
        fn_state.gs_9537 = s_34_0;
        // N s_34_2: jump b33
        return block_33(state, tracer, fn_state);
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_35_0: const #() : ()
        let s_35_0: () = ();
        // S s_35_1: call HDCR_read(s_35_0)
        let s_35_1: ProductType700c18a878c5601b = HDCR_read(state, tracer, s_35_0);
        // S s_35_2: call _get_HDCR_Type_TDE(s_35_1)
        let s_35_2: bool = u_get_HDCR_Type_TDE(state, tracer, s_35_1);
        // S s_35_3: cast zx s_35_2 -> bv
        let s_35_3: Bits = Bits::new(s_35_2 as u128, 1u16);
        // C s_35_4: const #1u : u8
        let s_35_4: bool = true;
        // C s_35_5: cast zx s_35_4 -> bv
        let s_35_5: Bits = Bits::new(s_35_4 as u128, 1u16);
        // S s_35_6: cmp-eq s_35_3 s_35_5
        let s_35_6: bool = ((s_35_3) == (s_35_5));
        // D s_35_7: write-var gs#9535 <= s_35_6
        fn_state.gs_9535 = s_35_6;
        // N s_35_8: jump b30
        return block_30(state, tracer, fn_state);
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_36_0: const #1u : u8
        let s_36_0: bool = true;
        // D s_36_1: write-var gs#9536 <= s_36_0
        fn_state.gs_9536 = s_36_0;
        // N s_36_2: jump b31
        return block_31(state, tracer, fn_state);
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_37_0: read-var fault:struct
        let s_37_0: ProductType1d757adad216cdef = fn_state.fault;
        // D s_37_1: call IsExternalAbort__1(s_37_0)
        let s_37_1: bool = IsExternalAbort__1(state, tracer, s_37_0);
        // D s_37_2: write-var gs#9533 <= s_37_1
        fn_state.gs_9533 = s_37_1;
        // N s_37_3: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_38_0: const #() : ()
        let s_38_0: () = ();
        // S s_38_1: call HCR2_read(s_38_0)
        let s_38_1: ProductType700c18a878c5601b = HCR2_read(state, tracer, s_38_0);
        // S s_38_2: call _get_HCR2_Type_TEA(s_38_1)
        let s_38_2: bool = u_get_HCR2_Type_TEA(state, tracer, s_38_1);
        // S s_38_3: cast zx s_38_2 -> bv
        let s_38_3: Bits = Bits::new(s_38_2 as u128, 1u16);
        // C s_38_4: const #1u : u8
        let s_38_4: bool = true;
        // C s_38_5: cast zx s_38_4 -> bv
        let s_38_5: Bits = Bits::new(s_38_4 as u128, 1u16);
        // S s_38_6: cmp-eq s_38_3 s_38_5
        let s_38_6: bool = ((s_38_3) == (s_38_5));
        // D s_38_7: write-var gs#9532 <= s_38_6
        fn_state.gs_9532 = s_38_6;
        // N s_38_8: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_39_0: const #1u : u8
        let s_39_0: bool = true;
        // D s_39_1: write-var gs#9534 <= s_39_0
        fn_state.gs_9534 = s_39_0;
        // N s_39_2: jump b27
        return block_27(state, tracer, fn_state);
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_40_0: const #16975u : u32
        let s_40_0: u32 = 16975;
        // D s_40_1: read-reg s_40_0:u8
        let s_40_1: u8 = {
            let value = state.read_register::<u8>(s_40_0 as isize);
            tracer.read_register(s_40_0 as isize, value);
            value
        };
        // D s_40_2: cast zx s_40_1 -> bv
        let s_40_2: Bits = Bits::new(s_40_1 as u128, 2u16);
        // C s_40_3: const #448u : u32
        let s_40_3: u32 = 448;
        // D s_40_4: read-reg s_40_3:u8
        let s_40_4: u8 = {
            let value = state.read_register::<u8>(s_40_3 as isize);
            tracer.read_register(s_40_3 as isize, value);
            value
        };
        // D s_40_5: cast zx s_40_4 -> bv
        let s_40_5: Bits = Bits::new(s_40_4 as u128, 2u16);
        // D s_40_6: cmp-eq s_40_2 s_40_5
        let s_40_6: bool = ((s_40_2) == (s_40_5));
        // N s_40_7: branch s_40_6 b43 b41
        if s_40_6 {
            return block_43(state, tracer, fn_state);
        } else {
            return block_41(state, tracer, fn_state);
        };
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_41_0: const #16975u : u32
        let s_41_0: u32 = 16975;
        // D s_41_1: read-reg s_41_0:u8
        let s_41_1: u8 = {
            let value = state.read_register::<u8>(s_41_0 as isize);
            tracer.read_register(s_41_0 as isize, value);
            value
        };
        // D s_41_2: cast zx s_41_1 -> bv
        let s_41_2: Bits = Bits::new(s_41_1 as u128, 2u16);
        // C s_41_3: const #440u : u32
        let s_41_3: u32 = 440;
        // D s_41_4: read-reg s_41_3:u8
        let s_41_4: u8 = {
            let value = state.read_register::<u8>(s_41_3 as isize);
            tracer.read_register(s_41_3 as isize, value);
            value
        };
        // D s_41_5: cast zx s_41_4 -> bv
        let s_41_5: Bits = Bits::new(s_41_4 as u128, 2u16);
        // D s_41_6: cmp-eq s_41_2 s_41_5
        let s_41_6: bool = ((s_41_2) == (s_41_5));
        // D s_41_7: write-var gs#9530 <= s_41_6
        fn_state.gs_9530 = s_41_6;
        // N s_41_8: jump b42
        return block_42(state, tracer, fn_state);
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_42_0: read-var gs#9530:u8
        let s_42_0: bool = fn_state.gs_9530;
        // D s_42_1: write-var gs#9531 <= s_42_0
        fn_state.gs_9531 = s_42_0;
        // N s_42_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_43_0: const #1u : u8
        let s_43_0: bool = true;
        // D s_43_1: write-var gs#9530 <= s_43_0
        fn_state.gs_9530 = s_43_0;
        // N s_43_2: jump b42
        return block_42(state, tracer, fn_state);
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_44_0: read-var fault:struct
        let s_44_0: ProductType1d757adad216cdef = fn_state.fault;
        // D s_44_1: call IsExternalAbort__1(s_44_0)
        let s_44_1: bool = IsExternalAbort__1(state, tracer, s_44_0);
        // D s_44_2: write-var gs#9529 <= s_44_1
        fn_state.gs_9529 = s_44_1;
        // N s_44_3: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_45_0: const #() : ()
        let s_45_0: () = ();
        // S s_45_1: call EffectiveEA(s_45_0)
        let s_45_1: bool = EffectiveEA(state, tracer, s_45_0);
        // S s_45_2: cast zx s_45_1 -> bv
        let s_45_2: Bits = Bits::new(s_45_1 as u128, 1u16);
        // C s_45_3: const #1u : u8
        let s_45_3: bool = true;
        // C s_45_4: cast zx s_45_3 -> bv
        let s_45_4: Bits = Bits::new(s_45_3 as u128, 1u16);
        // S s_45_5: cmp-eq s_45_2 s_45_4
        let s_45_5: bool = ((s_45_2) == (s_45_4));
        // D s_45_6: write-var gs#9528 <= s_45_5
        fn_state.gs_9528 = s_45_5;
        // N s_45_7: jump b2
        return block_2(state, tracer, fn_state);
    }
}
