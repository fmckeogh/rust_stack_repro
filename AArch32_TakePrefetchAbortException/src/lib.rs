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
use AArch32_ReportPrefetchAbort::*;
use u_get_HCR_Type_TGE::*;
use AArch32_EnterHypMode::*;
use ThisInstrAddr::*;
use IsExternalAbort__1::*;
use IsSecondStage::*;
use ExceptionSyndrome::*;
use u_get_HCR2_Type_TEA::*;
use EffectiveEA::*;
use EL2Enabled::*;
use AArch32_AbortSyndrome::*;
use HaveRASExt::*;
use AArch32_EnterMode::*;
use common::*;
pub fn AArch32_TakePrefetchAbortException<T: Tracer>(
    state: &mut State,
    tracer: &T,
    vaddress: u32,
    fault: ProductType1d757adad216cdef,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        route_to_monitor: bool,
        route_to_hyp: bool,
        gs_9590: bool,
        gs_9583: bool,
        gs_9589: bool,
        except: ProductTypeb7f99f96751e17c4,
        gs_9582: bool,
        preferred_exception_return: u32,
        gs_9581: bool,
        gs_9584: bool,
        gs_9585: bool,
        gs_9598: bool,
        vect_offset: i64,
        gs_9586: bool,
        gs_9591: bool,
        gs_9587: bool,
        gs_9588: bool,
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
        // N s_0_4: branch s_0_3 b48 b1
        if s_0_3 {
            return block_48(state, tracer, fn_state);
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
        // D s_1_1: write-var gs#9581 <= s_1_0
        fn_state.gs_9581 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var gs#9581:u8
        let s_2_0: bool = fn_state.gs_9581;
        // N s_2_1: branch s_2_0 b47 b3
        if s_2_0 {
            return block_47(state, tracer, fn_state);
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
        // D s_3_1: write-var gs#9582 <= s_3_0
        fn_state.gs_9582 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var gs#9582:u8
        let s_4_0: bool = fn_state.gs_9582;
        // D s_4_1: write-var route_to_monitor <= s_4_0
        fn_state.route_to_monitor = s_4_0;
        // C s_4_2: const #() : ()
        let s_4_2: () = ();
        // S s_4_3: call EL2Enabled(s_4_2)
        let s_4_3: bool = EL2Enabled(state, tracer, s_4_2);
        // N s_4_4: branch s_4_3 b43 b5
        if s_4_3 {
            return block_43(state, tracer, fn_state);
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
        // D s_5_1: write-var gs#9584 <= s_5_0
        fn_state.gs_9584 = s_5_0;
        // N s_5_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var gs#9584:u8
        let s_6_0: bool = fn_state.gs_9584;
        // N s_6_1: branch s_6_0 b24 b7
        if s_6_0 {
            return block_24(state, tracer, fn_state);
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
        // D s_7_1: write-var gs#9591 <= s_7_0
        fn_state.gs_9591 = s_7_0;
        // N s_7_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var gs#9591:u8
        let s_8_0: bool = fn_state.gs_9591;
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
        // C s_8_7: const #12u : u8
        let s_8_7: u8 = 12;
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
        // N s_8_14: branch s_8_13 b23 b9
        if s_8_13 {
            return block_23(state, tracer, fn_state);
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
        // N s_10_1: branch s_10_0 b22 b11
        if s_10_0 {
            return block_22(state, tracer, fn_state);
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
        // N s_11_7: branch s_11_6 b21 b12
        if s_11_6 {
            return block_21(state, tracer, fn_state);
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
        // D s_12_1: write-var gs#9598 <= s_12_0
        fn_state.gs_9598 = s_12_0;
        // N s_12_2: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var gs#9598:u8
        let s_13_0: bool = fn_state.gs_9598;
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
        // D s_14_3: call AArch32_ReportPrefetchAbort(s_14_0, s_14_1, s_14_2)
        let s_14_3: () = AArch32_ReportPrefetchAbort(
            state,
            tracer,
            s_14_0,
            s_14_1,
            s_14_2,
        );
        // C s_14_4: const #4s : i64
        let s_14_4: i64 = 4;
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
        // D s_15_0: read-var fault.16:struct
        let s_15_0: u32 = fn_state.fault._16;
        // C s_15_1: const #2u : u32
        let s_15_1: u32 = 2;
        // D s_15_2: cmp-eq s_15_0 s_15_1
        let s_15_2: bool = ((s_15_0) == (s_15_1));
        // N s_15_3: branch s_15_2 b20 b16
        if s_15_2 {
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
        // C s_16_0: const #17u : u32
        let s_16_0: u32 = 17;
        // D s_16_1: read-var fault:struct
        let s_16_1: ProductType1d757adad216cdef = fn_state.fault;
        // D s_16_2: read-var vaddress:u32
        let s_16_2: u32 = fn_state.vaddress;
        // C s_16_3: const #432u : u32
        let s_16_3: u32 = 432;
        // D s_16_4: read-reg s_16_3:u8
        let s_16_4: u8 = {
            let value = state.read_register::<u8>(s_16_3 as isize);
            tracer.read_register(s_16_3 as isize, value);
            value
        };
        // D s_16_5: call AArch32_AbortSyndrome(s_16_0, s_16_1, s_16_2, s_16_4)
        let s_16_5: ProductTypeb7f99f96751e17c4 = AArch32_AbortSyndrome(
            state,
            tracer,
            s_16_0,
            s_16_1,
            s_16_2,
            s_16_4,
        );
        // D s_16_6: write-var except <= s_16_5
        fn_state.except = s_16_5;
        // N s_16_7: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_17_0: const #16975u : u32
        let s_17_0: u32 = 16975;
        // D s_17_1: read-reg s_17_0:u8
        let s_17_1: u8 = {
            let value = state.read_register::<u8>(s_17_0 as isize);
            tracer.read_register(s_17_0 as isize, value);
            value
        };
        // D s_17_2: cast zx s_17_1 -> bv
        let s_17_2: Bits = Bits::new(s_17_1 as u128, 2u16);
        // C s_17_3: const #432u : u32
        let s_17_3: u32 = 432;
        // D s_17_4: read-reg s_17_3:u8
        let s_17_4: u8 = {
            let value = state.read_register::<u8>(s_17_3 as isize);
            tracer.read_register(s_17_3 as isize, value);
            value
        };
        // D s_17_5: cast zx s_17_4 -> bv
        let s_17_5: Bits = Bits::new(s_17_4 as u128, 2u16);
        // D s_17_6: cmp-eq s_17_2 s_17_5
        let s_17_6: bool = ((s_17_2) == (s_17_5));
        // N s_17_7: branch s_17_6 b19 b18
        if s_17_6 {
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
        // C s_18_0: const #20u : u8
        let s_18_0: u8 = 20;
        // C s_18_1: cast zx s_18_0 -> bv
        let s_18_1: Bits = Bits::new(s_18_0 as u128, 8u16);
        // C s_18_2: cast zx s_18_1 -> i
        let s_18_2: i128 = (s_18_1.value() as i128);
        // C s_18_3: cast reint s_18_2 -> i64
        let s_18_3: i64 = (s_18_2 as i64);
        // C s_18_4: cast zx s_18_3 -> i
        let s_18_4: i128 = (i128::try_from(s_18_3).unwrap());
        // D s_18_5: read-var except:struct
        let s_18_5: ProductTypeb7f99f96751e17c4 = fn_state.except;
        // D s_18_6: read-var preferred_exception_return:u32
        let s_18_6: u32 = fn_state.preferred_exception_return;
        // D s_18_7: call AArch32_EnterHypMode(s_18_5, s_18_6, s_18_4)
        let s_18_7: () = AArch32_EnterHypMode(state, tracer, s_18_5, s_18_6, s_18_4);
        // N s_18_8: return
        return;
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_19_0: read-var vect_offset:i64
        let s_19_0: i64 = fn_state.vect_offset;
        // D s_19_1: cast zx s_19_0 -> i
        let s_19_1: i128 = (i128::try_from(s_19_0).unwrap());
        // D s_19_2: read-var except:struct
        let s_19_2: ProductTypeb7f99f96751e17c4 = fn_state.except;
        // D s_19_3: read-var preferred_exception_return:u32
        let s_19_3: u32 = fn_state.preferred_exception_return;
        // D s_19_4: call AArch32_EnterHypMode(s_19_2, s_19_3, s_19_1)
        let s_19_4: () = AArch32_EnterHypMode(state, tracer, s_19_2, s_19_3, s_19_1);
        // N s_19_5: return
        return;
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_20_0: const #18u : u32
        let s_20_0: u32 = 18;
        // S s_20_1: call ExceptionSyndrome(s_20_0)
        let s_20_1: ProductTypeb7f99f96751e17c4 = ExceptionSyndrome(
            state,
            tracer,
            s_20_0,
        );
        // D s_20_2: write-var except <= s_20_1
        fn_state.except = s_20_1;
        // C s_20_3: const #64s : i64
        let s_20_3: i64 = 64;
        // C s_20_4: cast zx s_20_3 -> i
        let s_20_4: i128 = (i128::try_from(s_20_3).unwrap());
        // S s_20_5: call ThisInstrAddr(s_20_4)
        let s_20_5: Bits = ThisInstrAddr(state, tracer, s_20_4);
        // S s_20_6: cast reint s_20_5 -> u64
        let s_20_6: u64 = (s_20_5.value() as u64);
        // D s_20_7: write-var except.9 <= s_20_6
        fn_state.except._9 = s_20_6;
        // N s_20_8: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_21_0: const #1u : u8
        let s_21_0: bool = true;
        // D s_21_1: write-var gs#9598 <= s_21_0
        fn_state.gs_9598 = s_21_0;
        // N s_21_2: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_22_0: read-var route_to_monitor:u8
        let s_22_0: bool = fn_state.route_to_monitor;
        // D s_22_1: read-var fault:struct
        let s_22_1: ProductType1d757adad216cdef = fn_state.fault;
        // D s_22_2: read-var vaddress:u32
        let s_22_2: u32 = fn_state.vaddress;
        // D s_22_3: call AArch32_ReportPrefetchAbort(s_22_0, s_22_1, s_22_2)
        let s_22_3: () = AArch32_ReportPrefetchAbort(
            state,
            tracer,
            s_22_0,
            s_22_1,
            s_22_2,
        );
        // C s_22_4: const #4s : i64
        let s_22_4: i64 = 4;
        // C s_22_5: cast zx s_22_4 -> i
        let s_22_5: i128 = (i128::try_from(s_22_4).unwrap());
        // D s_22_6: read-var vect_offset:i64
        let s_22_6: i64 = fn_state.vect_offset;
        // D s_22_7: cast zx s_22_6 -> i
        let s_22_7: i128 = (i128::try_from(s_22_6).unwrap());
        // D s_22_8: read-var preferred_exception_return:u32
        let s_22_8: u32 = fn_state.preferred_exception_return;
        // D s_22_9: call AArch32_EnterMonitorMode(s_22_8, s_22_5, s_22_7)
        let s_22_9: () = AArch32_EnterMonitorMode(state, tracer, s_22_8, s_22_5, s_22_7);
        // N s_22_10: return
        return;
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_23_0: const #() : ()
        let s_23_0: () = ();
        // S s_23_1: call DBGDSCRext_read(s_23_0)
        let s_23_1: ProductType700c18a878c5601b = DBGDSCRext_read(state, tracer, s_23_0);
        // D s_23_2: read-var fault.2:struct
        let s_23_2: u8 = fn_state.fault._2;
        // D s_23_3: call _update_DBGDSCRext_Type_MOE(s_23_1, s_23_2)
        let s_23_3: ProductType700c18a878c5601b = u_update_DBGDSCRext_Type_MOE(
            state,
            tracer,
            s_23_1,
            s_23_2,
        );
        // D s_23_4: call DBGDSCRext_write(s_23_3)
        let s_23_4: () = DBGDSCRext_write(state, tracer, s_23_3);
        // N s_23_5: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_24_0: const #() : ()
        let s_24_0: () = ();
        // S s_24_1: call HCR_read(s_24_0)
        let s_24_1: ProductType700c18a878c5601b = HCR_read(state, tracer, s_24_0);
        // S s_24_2: call _get_HCR_Type_TGE(s_24_1)
        let s_24_2: bool = u_get_HCR_Type_TGE(state, tracer, s_24_1);
        // S s_24_3: cast zx s_24_2 -> bv
        let s_24_3: Bits = Bits::new(s_24_2 as u128, 1u16);
        // C s_24_4: const #1u : u8
        let s_24_4: bool = true;
        // C s_24_5: cast zx s_24_4 -> bv
        let s_24_5: Bits = Bits::new(s_24_4 as u128, 1u16);
        // S s_24_6: cmp-eq s_24_3 s_24_5
        let s_24_6: bool = ((s_24_3) == (s_24_5));
        // N s_24_7: branch s_24_6 b42 b25
        if s_24_6 {
            return block_42(state, tracer, fn_state);
        } else {
            return block_25(state, tracer, fn_state);
        };
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_25_0: const #() : ()
        let s_25_0: () = ();
        // S s_25_1: call HaveRASExt(s_25_0)
        let s_25_1: bool = HaveRASExt(state, tracer, s_25_0);
        // N s_25_2: branch s_25_1 b41 b26
        if s_25_1 {
            return block_41(state, tracer, fn_state);
        } else {
            return block_26(state, tracer, fn_state);
        };
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_26_0: const #0u : u8
        let s_26_0: bool = false;
        // D s_26_1: write-var gs#9585 <= s_26_0
        fn_state.gs_9585 = s_26_0;
        // N s_26_2: jump b27
        return block_27(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_27_0: read-var gs#9585:u8
        let s_27_0: bool = fn_state.gs_9585;
        // N s_27_1: branch s_27_0 b40 b28
        if s_27_0 {
            return block_40(state, tracer, fn_state);
        } else {
            return block_28(state, tracer, fn_state);
        };
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_28_0: const #0u : u8
        let s_28_0: bool = false;
        // D s_28_1: write-var gs#9586 <= s_28_0
        fn_state.gs_9586 = s_28_0;
        // N s_28_2: jump b29
        return block_29(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_29_0: read-var gs#9586:u8
        let s_29_0: bool = fn_state.gs_9586;
        // D s_29_1: write-var gs#9587 <= s_29_0
        fn_state.gs_9587 = s_29_0;
        // N s_29_2: jump b30
        return block_30(state, tracer, fn_state);
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_30_0: read-var gs#9587:u8
        let s_30_0: bool = fn_state.gs_9587;
        // N s_30_1: branch s_30_0 b39 b31
        if s_30_0 {
            return block_39(state, tracer, fn_state);
        } else {
            return block_31(state, tracer, fn_state);
        };
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_31_0: read-var fault:struct
        let s_31_0: ProductType1d757adad216cdef = fn_state.fault;
        // D s_31_1: call IsDebugException(s_31_0)
        let s_31_1: bool = IsDebugException(state, tracer, s_31_0);
        // N s_31_2: branch s_31_1 b38 b32
        if s_31_1 {
            return block_38(state, tracer, fn_state);
        } else {
            return block_32(state, tracer, fn_state);
        };
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_32_0: const #0u : u8
        let s_32_0: bool = false;
        // D s_32_1: write-var gs#9588 <= s_32_0
        fn_state.gs_9588 = s_32_0;
        // N s_32_2: jump b33
        return block_33(state, tracer, fn_state);
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_33_0: read-var gs#9588:u8
        let s_33_0: bool = fn_state.gs_9588;
        // D s_33_1: write-var gs#9589 <= s_33_0
        fn_state.gs_9589 = s_33_0;
        // N s_33_2: jump b34
        return block_34(state, tracer, fn_state);
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_34_0: read-var gs#9589:u8
        let s_34_0: bool = fn_state.gs_9589;
        // N s_34_1: branch s_34_0 b37 b35
        if s_34_0 {
            return block_37(state, tracer, fn_state);
        } else {
            return block_35(state, tracer, fn_state);
        };
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_35_0: read-var fault:struct
        let s_35_0: ProductType1d757adad216cdef = fn_state.fault;
        // D s_35_1: call IsSecondStage(s_35_0)
        let s_35_1: bool = IsSecondStage(state, tracer, s_35_0);
        // D s_35_2: write-var gs#9590 <= s_35_1
        fn_state.gs_9590 = s_35_1;
        // N s_35_3: jump b36
        return block_36(state, tracer, fn_state);
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_36_0: read-var gs#9590:u8
        let s_36_0: bool = fn_state.gs_9590;
        // D s_36_1: write-var gs#9591 <= s_36_0
        fn_state.gs_9591 = s_36_0;
        // N s_36_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_37_0: const #1u : u8
        let s_37_0: bool = true;
        // D s_37_1: write-var gs#9590 <= s_37_0
        fn_state.gs_9590 = s_37_0;
        // N s_37_2: jump b36
        return block_36(state, tracer, fn_state);
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_38_0: const #() : ()
        let s_38_0: () = ();
        // S s_38_1: call HDCR_read(s_38_0)
        let s_38_1: ProductType700c18a878c5601b = HDCR_read(state, tracer, s_38_0);
        // S s_38_2: call _get_HDCR_Type_TDE(s_38_1)
        let s_38_2: bool = u_get_HDCR_Type_TDE(state, tracer, s_38_1);
        // S s_38_3: cast zx s_38_2 -> bv
        let s_38_3: Bits = Bits::new(s_38_2 as u128, 1u16);
        // C s_38_4: const #1u : u8
        let s_38_4: bool = true;
        // C s_38_5: cast zx s_38_4 -> bv
        let s_38_5: Bits = Bits::new(s_38_4 as u128, 1u16);
        // S s_38_6: cmp-eq s_38_3 s_38_5
        let s_38_6: bool = ((s_38_3) == (s_38_5));
        // D s_38_7: write-var gs#9588 <= s_38_6
        fn_state.gs_9588 = s_38_6;
        // N s_38_8: jump b33
        return block_33(state, tracer, fn_state);
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_39_0: const #1u : u8
        let s_39_0: bool = true;
        // D s_39_1: write-var gs#9589 <= s_39_0
        fn_state.gs_9589 = s_39_0;
        // N s_39_2: jump b34
        return block_34(state, tracer, fn_state);
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_40_0: read-var fault:struct
        let s_40_0: ProductType1d757adad216cdef = fn_state.fault;
        // D s_40_1: call IsExternalAbort__1(s_40_0)
        let s_40_1: bool = IsExternalAbort__1(state, tracer, s_40_0);
        // D s_40_2: write-var gs#9586 <= s_40_1
        fn_state.gs_9586 = s_40_1;
        // N s_40_3: jump b29
        return block_29(state, tracer, fn_state);
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_41_0: const #() : ()
        let s_41_0: () = ();
        // S s_41_1: call HCR2_read(s_41_0)
        let s_41_1: ProductType700c18a878c5601b = HCR2_read(state, tracer, s_41_0);
        // S s_41_2: call _get_HCR2_Type_TEA(s_41_1)
        let s_41_2: bool = u_get_HCR2_Type_TEA(state, tracer, s_41_1);
        // S s_41_3: cast zx s_41_2 -> bv
        let s_41_3: Bits = Bits::new(s_41_2 as u128, 1u16);
        // C s_41_4: const #1u : u8
        let s_41_4: bool = true;
        // C s_41_5: cast zx s_41_4 -> bv
        let s_41_5: Bits = Bits::new(s_41_4 as u128, 1u16);
        // S s_41_6: cmp-eq s_41_3 s_41_5
        let s_41_6: bool = ((s_41_3) == (s_41_5));
        // D s_41_7: write-var gs#9585 <= s_41_6
        fn_state.gs_9585 = s_41_6;
        // N s_41_8: jump b27
        return block_27(state, tracer, fn_state);
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_42_0: const #1u : u8
        let s_42_0: bool = true;
        // D s_42_1: write-var gs#9587 <= s_42_0
        fn_state.gs_9587 = s_42_0;
        // N s_42_2: jump b30
        return block_30(state, tracer, fn_state);
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_43_0: const #16975u : u32
        let s_43_0: u32 = 16975;
        // D s_43_1: read-reg s_43_0:u8
        let s_43_1: u8 = {
            let value = state.read_register::<u8>(s_43_0 as isize);
            tracer.read_register(s_43_0 as isize, value);
            value
        };
        // D s_43_2: cast zx s_43_1 -> bv
        let s_43_2: Bits = Bits::new(s_43_1 as u128, 2u16);
        // C s_43_3: const #448u : u32
        let s_43_3: u32 = 448;
        // D s_43_4: read-reg s_43_3:u8
        let s_43_4: u8 = {
            let value = state.read_register::<u8>(s_43_3 as isize);
            tracer.read_register(s_43_3 as isize, value);
            value
        };
        // D s_43_5: cast zx s_43_4 -> bv
        let s_43_5: Bits = Bits::new(s_43_4 as u128, 2u16);
        // D s_43_6: cmp-eq s_43_2 s_43_5
        let s_43_6: bool = ((s_43_2) == (s_43_5));
        // N s_43_7: branch s_43_6 b46 b44
        if s_43_6 {
            return block_46(state, tracer, fn_state);
        } else {
            return block_44(state, tracer, fn_state);
        };
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_44_0: const #16975u : u32
        let s_44_0: u32 = 16975;
        // D s_44_1: read-reg s_44_0:u8
        let s_44_1: u8 = {
            let value = state.read_register::<u8>(s_44_0 as isize);
            tracer.read_register(s_44_0 as isize, value);
            value
        };
        // D s_44_2: cast zx s_44_1 -> bv
        let s_44_2: Bits = Bits::new(s_44_1 as u128, 2u16);
        // C s_44_3: const #440u : u32
        let s_44_3: u32 = 440;
        // D s_44_4: read-reg s_44_3:u8
        let s_44_4: u8 = {
            let value = state.read_register::<u8>(s_44_3 as isize);
            tracer.read_register(s_44_3 as isize, value);
            value
        };
        // D s_44_5: cast zx s_44_4 -> bv
        let s_44_5: Bits = Bits::new(s_44_4 as u128, 2u16);
        // D s_44_6: cmp-eq s_44_2 s_44_5
        let s_44_6: bool = ((s_44_2) == (s_44_5));
        // D s_44_7: write-var gs#9583 <= s_44_6
        fn_state.gs_9583 = s_44_6;
        // N s_44_8: jump b45
        return block_45(state, tracer, fn_state);
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_45_0: read-var gs#9583:u8
        let s_45_0: bool = fn_state.gs_9583;
        // D s_45_1: write-var gs#9584 <= s_45_0
        fn_state.gs_9584 = s_45_0;
        // N s_45_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_46_0: const #1u : u8
        let s_46_0: bool = true;
        // D s_46_1: write-var gs#9583 <= s_46_0
        fn_state.gs_9583 = s_46_0;
        // N s_46_2: jump b45
        return block_45(state, tracer, fn_state);
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_47_0: read-var fault:struct
        let s_47_0: ProductType1d757adad216cdef = fn_state.fault;
        // D s_47_1: call IsExternalAbort__1(s_47_0)
        let s_47_1: bool = IsExternalAbort__1(state, tracer, s_47_0);
        // D s_47_2: write-var gs#9582 <= s_47_1
        fn_state.gs_9582 = s_47_1;
        // N s_47_3: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_48_0: const #() : ()
        let s_48_0: () = ();
        // S s_48_1: call EffectiveEA(s_48_0)
        let s_48_1: bool = EffectiveEA(state, tracer, s_48_0);
        // S s_48_2: cast zx s_48_1 -> bv
        let s_48_2: Bits = Bits::new(s_48_1 as u128, 1u16);
        // C s_48_3: const #1u : u8
        let s_48_3: bool = true;
        // C s_48_4: cast zx s_48_3 -> bv
        let s_48_4: Bits = Bits::new(s_48_3 as u128, 1u16);
        // S s_48_5: cmp-eq s_48_2 s_48_4
        let s_48_5: bool = ((s_48_2) == (s_48_4));
        // D s_48_6: write-var gs#9581 <= s_48_5
        fn_state.gs_9581 = s_48_5;
        // N s_48_7: jump b2
        return block_2(state, tracer, fn_state);
    }
}
