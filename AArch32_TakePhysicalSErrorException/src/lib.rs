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
use u_get_HCR_Type_AMO::*;
use u_get_HCR_EL2_Type_TGE::*;
use ClearPendingPhysicalSError::*;
use IsInHost::*;
use u_get_HCR_EL2_Type_AMO::*;
use HCR_read::*;
use GetPendingPhysicalSError::*;
use IsSErrorEdgeTriggered::*;
use AArch64_TakePhysicalSErrorException::*;
use AArch32_EnterMonitorMode::*;
use AArch32_ReportDataAbort::*;
use u_get_HCR_Type_TGE::*;
use Unreachable::*;
use ThisInstrAddr::*;
use u__UNKNOWN_bits::*;
use AArch32_EnterHypMode::*;
use ELUsingAArch32::*;
use EffectiveEA::*;
use EL2Enabled::*;
use AArch32_AbortSyndrome::*;
use u_get_SCR_Type_EA::*;
use AArch32_EnterMode::*;
use common::*;
pub fn AArch32_TakePhysicalSErrorException<T: Tracer>(
    state: &mut State,
    tracer: &T,
    implicit_esb: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_31947: bool,
        route_to_monitor: bool,
        fault: ProductType1d757adad216cdef,
        gs_31955: bool,
        gs_31957: bool,
        route_to_aarch64: bool,
        gs_31944: bool,
        gs_31953: bool,
        target_el: u8,
        gs_31954: bool,
        route_to_hyp: bool,
        gs_31951: bool,
        gs_31952: bool,
        except: ProductTypeb7f99f96751e17c4,
        preferred_exception_return: u32,
        gs_31956: bool,
        vaddress: u32,
        vect_offset: i64,
        gs_31963: bool,
        gs_31943: bool,
        gs_31946: bool,
        gs_31945: bool,
        implicit_esb: bool,
    }
    let fn_state = FunctionState {
        implicit_esb,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_0_0: const #16975u : u32
        let s_0_0: u32 = 16975;
        // D s_0_1: read-reg s_0_0:u8
        let s_0_1: u8 = {
            let value = state.read_register::<u8>(s_0_0 as isize);
            tracer.read_register(s_0_0 as isize, value);
            value
        };
        // D s_0_2: cast zx s_0_1 -> bv
        let s_0_2: Bits = Bits::new(s_0_1 as u128, 2u16);
        // C s_0_3: const #448u : u32
        let s_0_3: u32 = 448;
        // D s_0_4: read-reg s_0_3:u8
        let s_0_4: u8 = {
            let value = state.read_register::<u8>(s_0_3 as isize);
            tracer.read_register(s_0_3 as isize, value);
            value
        };
        // D s_0_5: cast zx s_0_4 -> bv
        let s_0_5: Bits = Bits::new(s_0_4 as u128, 2u16);
        // D s_0_6: cmp-eq s_0_2 s_0_5
        let s_0_6: bool = ((s_0_2) == (s_0_5));
        // N s_0_7: branch s_0_6 b64 b1
        if s_0_6 {
            return block_64(state, tracer, fn_state);
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
        // D s_1_1: write-var gs#31943 <= s_1_0
        fn_state.gs_31943 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var gs#31943:u8
        let s_2_0: bool = fn_state.gs_31943;
        // D s_2_1: write-var route_to_aarch64 <= s_2_0
        fn_state.route_to_aarch64 = s_2_0;
        // D s_2_2: read-var route_to_aarch64:u8
        let s_2_2: bool = fn_state.route_to_aarch64;
        // D s_2_3: not s_2_2
        let s_2_3: bool = !s_2_2;
        // N s_2_4: branch s_2_3 b63 b3
        if s_2_3 {
            return block_63(state, tracer, fn_state);
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
        // D s_3_1: write-var gs#31944 <= s_3_0
        fn_state.gs_31944 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var gs#31944:u8
        let s_4_0: bool = fn_state.gs_31944;
        // N s_4_1: branch s_4_0 b62 b5
        if s_4_0 {
            return block_62(state, tracer, fn_state);
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
        // D s_5_1: write-var gs#31945 <= s_5_0
        fn_state.gs_31945 = s_5_0;
        // N s_5_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var gs#31945:u8
        let s_6_0: bool = fn_state.gs_31945;
        // N s_6_1: branch s_6_0 b55 b7
        if s_6_0 {
            return block_55(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_7_0: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var route_to_aarch64:u8
        let s_8_0: bool = fn_state.route_to_aarch64;
        // D s_8_1: not s_8_0
        let s_8_1: bool = !s_8_0;
        // N s_8_2: branch s_8_1 b54 b9
        if s_8_1 {
            return block_54(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_9_0: const #0u : u8
        let s_9_0: bool = false;
        // D s_9_1: write-var gs#31946 <= s_9_0
        fn_state.gs_31946 = s_9_0;
        // N s_9_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var gs#31946:u8
        let s_10_0: bool = fn_state.gs_31946;
        // N s_10_1: branch s_10_0 b53 b11
        if s_10_0 {
            return block_53(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_11_0: const #0u : u8
        let s_11_0: bool = false;
        // D s_11_1: write-var gs#31947 <= s_11_0
        fn_state.gs_31947 = s_11_0;
        // N s_11_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var gs#31947:u8
        let s_12_0: bool = fn_state.gs_31947;
        // N s_12_1: branch s_12_0 b52 b13
        if s_12_0 {
            return block_52(state, tracer, fn_state);
        } else {
            return block_13(state, tracer, fn_state);
        };
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_13_0: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var route_to_aarch64:u8
        let s_14_0: bool = fn_state.route_to_aarch64;
        // N s_14_1: branch s_14_0 b51 b15
        if s_14_0 {
            return block_51(state, tracer, fn_state);
        } else {
            return block_15(state, tracer, fn_state);
        };
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_15_0: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_16_0: const #424u : u32
        let s_16_0: u32 = 424;
        // D s_16_1: read-reg s_16_0:u8
        let s_16_1: u8 = {
            let value = state.read_register::<u8>(s_16_0 as isize);
            tracer.read_register(s_16_0 as isize, value);
            value
        };
        // C s_16_2: const #2u : u8
        let s_16_2: u8 = 2;
        // D s_16_3: cmp-lt s_16_1 s_16_2
        let s_16_3: bool = ((s_16_1) < (s_16_2));
        // N s_16_4: branch s_16_3 b50 b17
        if s_16_3 {
            return block_50(state, tracer, fn_state);
        } else {
            return block_17(state, tracer, fn_state);
        };
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_17_0: const #0u : u8
        let s_17_0: bool = false;
        // D s_17_1: write-var gs#31953 <= s_17_0
        fn_state.gs_31953 = s_17_0;
        // N s_17_2: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_18_0: read-var gs#31953:u8
        let s_18_0: bool = fn_state.gs_31953;
        // D s_18_1: write-var route_to_monitor <= s_18_0
        fn_state.route_to_monitor = s_18_0;
        // C s_18_2: const #16975u : u32
        let s_18_2: u32 = 16975;
        // D s_18_3: read-reg s_18_2:u8
        let s_18_3: u8 = {
            let value = state.read_register::<u8>(s_18_2 as isize);
            tracer.read_register(s_18_2 as isize, value);
            value
        };
        // D s_18_4: cast zx s_18_3 -> bv
        let s_18_4: Bits = Bits::new(s_18_3 as u128, 2u16);
        // C s_18_5: const #448u : u32
        let s_18_5: u32 = 448;
        // D s_18_6: read-reg s_18_5:u8
        let s_18_6: u8 = {
            let value = state.read_register::<u8>(s_18_5 as isize);
            tracer.read_register(s_18_5 as isize, value);
            value
        };
        // D s_18_7: cast zx s_18_6 -> bv
        let s_18_7: Bits = Bits::new(s_18_6 as u128, 2u16);
        // D s_18_8: cmp-eq s_18_4 s_18_7
        let s_18_8: bool = ((s_18_4) == (s_18_7));
        // N s_18_9: branch s_18_8 b49 b19
        if s_18_8 {
            return block_49(state, tracer, fn_state);
        } else {
            return block_19(state, tracer, fn_state);
        };
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_19_0: const #16975u : u32
        let s_19_0: u32 = 16975;
        // D s_19_1: read-reg s_19_0:u8
        let s_19_1: u8 = {
            let value = state.read_register::<u8>(s_19_0 as isize);
            tracer.read_register(s_19_0 as isize, value);
            value
        };
        // D s_19_2: cast zx s_19_1 -> bv
        let s_19_2: Bits = Bits::new(s_19_1 as u128, 2u16);
        // C s_19_3: const #440u : u32
        let s_19_3: u32 = 440;
        // D s_19_4: read-reg s_19_3:u8
        let s_19_4: u8 = {
            let value = state.read_register::<u8>(s_19_3 as isize);
            tracer.read_register(s_19_3 as isize, value);
            value
        };
        // D s_19_5: cast zx s_19_4 -> bv
        let s_19_5: Bits = Bits::new(s_19_4 as u128, 2u16);
        // D s_19_6: cmp-eq s_19_2 s_19_5
        let s_19_6: bool = ((s_19_2) == (s_19_5));
        // D s_19_7: write-var gs#31954 <= s_19_6
        fn_state.gs_31954 = s_19_6;
        // N s_19_8: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_20_0: read-var gs#31954:u8
        let s_20_0: bool = fn_state.gs_31954;
        // N s_20_1: branch s_20_0 b48 b21
        if s_20_0 {
            return block_48(state, tracer, fn_state);
        } else {
            return block_21(state, tracer, fn_state);
        };
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_21_0: const #0u : u8
        let s_21_0: bool = false;
        // D s_21_1: write-var gs#31955 <= s_21_0
        fn_state.gs_31955 = s_21_0;
        // N s_21_2: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_22_0: read-var gs#31955:u8
        let s_22_0: bool = fn_state.gs_31955;
        // N s_22_1: branch s_22_0 b44 b23
        if s_22_0 {
            return block_44(state, tracer, fn_state);
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
        // D s_23_1: write-var gs#31957 <= s_23_0
        fn_state.gs_31957 = s_23_0;
        // N s_23_2: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_24_0: read-var gs#31957:u8
        let s_24_0: bool = fn_state.gs_31957;
        // D s_24_1: write-var route_to_hyp <= s_24_0
        fn_state.route_to_hyp = s_24_0;
        // C s_24_2: const #32s : i64
        let s_24_2: i64 = 32;
        // C s_24_3: cast zx s_24_2 -> i
        let s_24_3: i128 = (i128::try_from(s_24_2).unwrap());
        // S s_24_4: call ThisInstrAddr(s_24_3)
        let s_24_4: Bits = ThisInstrAddr(state, tracer, s_24_3);
        // S s_24_5: cast reint s_24_4 -> u32
        let s_24_5: u32 = (s_24_4.value() as u32);
        // D s_24_6: write-var preferred_exception_return <= s_24_5
        fn_state.preferred_exception_return = s_24_5;
        // C s_24_7: const #16u : u8
        let s_24_7: u8 = 16;
        // C s_24_8: cast zx s_24_7 -> bv
        let s_24_8: Bits = Bits::new(s_24_7 as u128, 8u16);
        // C s_24_9: cast zx s_24_8 -> i
        let s_24_9: i128 = (s_24_8.value() as i128);
        // C s_24_10: cast reint s_24_9 -> i64
        let s_24_10: i64 = (s_24_9 as i64);
        // D s_24_11: write-var vect_offset <= s_24_10
        fn_state.vect_offset = s_24_10;
        // D s_24_12: read-var route_to_monitor:u8
        let s_24_12: bool = fn_state.route_to_monitor;
        // N s_24_13: branch s_24_12 b43 b25
        if s_24_12 {
            return block_43(state, tracer, fn_state);
        } else {
            return block_25(state, tracer, fn_state);
        };
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_25_0: const #16975u : u32
        let s_25_0: u32 = 16975;
        // D s_25_1: read-reg s_25_0:u8
        let s_25_1: u8 = {
            let value = state.read_register::<u8>(s_25_0 as isize);
            tracer.read_register(s_25_0 as isize, value);
            value
        };
        // D s_25_2: cast zx s_25_1 -> bv
        let s_25_2: Bits = Bits::new(s_25_1 as u128, 2u16);
        // C s_25_3: const #432u : u32
        let s_25_3: u32 = 432;
        // D s_25_4: read-reg s_25_3:u8
        let s_25_4: u8 = {
            let value = state.read_register::<u8>(s_25_3 as isize);
            tracer.read_register(s_25_3 as isize, value);
            value
        };
        // D s_25_5: cast zx s_25_4 -> bv
        let s_25_5: Bits = Bits::new(s_25_4 as u128, 2u16);
        // D s_25_6: cmp-eq s_25_2 s_25_5
        let s_25_6: bool = ((s_25_2) == (s_25_5));
        // N s_25_7: branch s_25_6 b42 b26
        if s_25_6 {
            return block_42(state, tracer, fn_state);
        } else {
            return block_26(state, tracer, fn_state);
        };
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_26_0: read-var route_to_hyp:u8
        let s_26_0: bool = fn_state.route_to_hyp;
        // D s_26_1: write-var gs#31963 <= s_26_0
        fn_state.gs_31963 = s_26_0;
        // N s_26_2: jump b27
        return block_27(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_27_0: read-var gs#31963:u8
        let s_27_0: bool = fn_state.gs_31963;
        // N s_27_1: branch s_27_0 b41 b28
        if s_27_0 {
            return block_41(state, tracer, fn_state);
        } else {
            return block_28(state, tracer, fn_state);
        };
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_28_0: const #440u : u32
        let s_28_0: u32 = 440;
        // D s_28_1: read-reg s_28_0:u8
        let s_28_1: u8 = {
            let value = state.read_register::<u8>(s_28_0 as isize);
            tracer.read_register(s_28_0 as isize, value);
            value
        };
        // D s_28_2: write-var target_el <= s_28_1
        fn_state.target_el = s_28_1;
        // N s_28_3: jump b29
        return block_29(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_29_0: const #() : ()
        let s_29_0: () = ();
        // S s_29_1: call GetPendingPhysicalSError(s_29_0)
        let s_29_1: ProductType1d757adad216cdef = GetPendingPhysicalSError(
            state,
            tracer,
            s_29_0,
        );
        // D s_29_2: write-var fault <= s_29_1
        fn_state.fault = s_29_1;
        // C s_29_3: const #32s : i64
        let s_29_3: i64 = 32;
        // C s_29_4: cast zx s_29_3 -> i
        let s_29_4: i128 = (i128::try_from(s_29_3).unwrap());
        // S s_29_5: call __UNKNOWN_bits(s_29_4)
        let s_29_5: Bits = u__UNKNOWN_bits(state, tracer, s_29_4);
        // S s_29_6: cast reint s_29_5 -> u32
        let s_29_6: u32 = (s_29_5.value() as u32);
        // D s_29_7: write-var vaddress <= s_29_6
        fn_state.vaddress = s_29_6;
        // C s_29_8: const #19u : u32
        let s_29_8: u32 = 19;
        // D s_29_9: read-var fault:struct
        let s_29_9: ProductType1d757adad216cdef = fn_state.fault;
        // D s_29_10: read-var vaddress:u32
        let s_29_10: u32 = fn_state.vaddress;
        // D s_29_11: read-var target_el:u8
        let s_29_11: u8 = fn_state.target_el;
        // D s_29_12: call AArch32_AbortSyndrome(s_29_8, s_29_9, s_29_10, s_29_11)
        let s_29_12: ProductTypeb7f99f96751e17c4 = AArch32_AbortSyndrome(
            state,
            tracer,
            s_29_8,
            s_29_9,
            s_29_10,
            s_29_11,
        );
        // D s_29_13: write-var except <= s_29_12
        fn_state.except = s_29_12;
        // C s_29_14: const #() : ()
        let s_29_14: () = ();
        // S s_29_15: call IsSErrorEdgeTriggered(s_29_14)
        let s_29_15: bool = IsSErrorEdgeTriggered(state, tracer, s_29_14);
        // N s_29_16: branch s_29_15 b40 b30
        if s_29_15 {
            return block_40(state, tracer, fn_state);
        } else {
            return block_30(state, tracer, fn_state);
        };
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_30_0: jump b31
        return block_31(state, tracer, fn_state);
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_31_0: read-var target_el:u8
        let s_31_0: u8 = fn_state.target_el;
        // D s_31_1: cast zx s_31_0 -> bv
        let s_31_1: Bits = Bits::new(s_31_0 as u128, 2u16);
        // C s_31_2: const #424u : u32
        let s_31_2: u32 = 424;
        // D s_31_3: read-reg s_31_2:u8
        let s_31_3: u8 = {
            let value = state.read_register::<u8>(s_31_2 as isize);
            tracer.read_register(s_31_2 as isize, value);
            value
        };
        // D s_31_4: cast zx s_31_3 -> bv
        let s_31_4: Bits = Bits::new(s_31_3 as u128, 2u16);
        // D s_31_5: cmp-eq s_31_1 s_31_4
        let s_31_5: bool = ((s_31_1) == (s_31_4));
        // D s_31_6: not s_31_5
        let s_31_6: bool = !s_31_5;
        // N s_31_7: branch s_31_6 b33 b32
        if s_31_6 {
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
        // D s_32_0: read-var route_to_monitor:u8
        let s_32_0: bool = fn_state.route_to_monitor;
        // D s_32_1: read-var fault:struct
        let s_32_1: ProductType1d757adad216cdef = fn_state.fault;
        // D s_32_2: read-var vaddress:u32
        let s_32_2: u32 = fn_state.vaddress;
        // D s_32_3: call AArch32_ReportDataAbort(s_32_0, s_32_1, s_32_2)
        let s_32_3: () = AArch32_ReportDataAbort(state, tracer, s_32_0, s_32_1, s_32_2);
        // C s_32_4: const #8s : i64
        let s_32_4: i64 = 8;
        // C s_32_5: cast zx s_32_4 -> i
        let s_32_5: i128 = (i128::try_from(s_32_4).unwrap());
        // D s_32_6: read-var vect_offset:i64
        let s_32_6: i64 = fn_state.vect_offset;
        // D s_32_7: cast zx s_32_6 -> i
        let s_32_7: i128 = (i128::try_from(s_32_6).unwrap());
        // D s_32_8: read-var preferred_exception_return:u32
        let s_32_8: u32 = fn_state.preferred_exception_return;
        // D s_32_9: call AArch32_EnterMonitorMode(s_32_8, s_32_5, s_32_7)
        let s_32_9: () = AArch32_EnterMonitorMode(state, tracer, s_32_8, s_32_5, s_32_7);
        // N s_32_10: return
        return;
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_33_0: read-var target_el:u8
        let s_33_0: u8 = fn_state.target_el;
        // D s_33_1: cast zx s_33_0 -> bv
        let s_33_1: Bits = Bits::new(s_33_0 as u128, 2u16);
        // C s_33_2: const #432u : u32
        let s_33_2: u32 = 432;
        // D s_33_3: read-reg s_33_2:u8
        let s_33_3: u8 = {
            let value = state.read_register::<u8>(s_33_2 as isize);
            tracer.read_register(s_33_2 as isize, value);
            value
        };
        // D s_33_4: cast zx s_33_3 -> bv
        let s_33_4: Bits = Bits::new(s_33_3 as u128, 2u16);
        // D s_33_5: cmp-eq s_33_1 s_33_4
        let s_33_5: bool = ((s_33_1) == (s_33_4));
        // D s_33_6: not s_33_5
        let s_33_6: bool = !s_33_5;
        // N s_33_7: branch s_33_6 b37 b34
        if s_33_6 {
            return block_37(state, tracer, fn_state);
        } else {
            return block_34(state, tracer, fn_state);
        };
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_34_0: const #16975u : u32
        let s_34_0: u32 = 16975;
        // D s_34_1: read-reg s_34_0:u8
        let s_34_1: u8 = {
            let value = state.read_register::<u8>(s_34_0 as isize);
            tracer.read_register(s_34_0 as isize, value);
            value
        };
        // D s_34_2: cast zx s_34_1 -> bv
        let s_34_2: Bits = Bits::new(s_34_1 as u128, 2u16);
        // C s_34_3: const #432u : u32
        let s_34_3: u32 = 432;
        // D s_34_4: read-reg s_34_3:u8
        let s_34_4: u8 = {
            let value = state.read_register::<u8>(s_34_3 as isize);
            tracer.read_register(s_34_3 as isize, value);
            value
        };
        // D s_34_5: cast zx s_34_4 -> bv
        let s_34_5: Bits = Bits::new(s_34_4 as u128, 2u16);
        // D s_34_6: cmp-eq s_34_2 s_34_5
        let s_34_6: bool = ((s_34_2) == (s_34_5));
        // N s_34_7: branch s_34_6 b36 b35
        if s_34_6 {
            return block_36(state, tracer, fn_state);
        } else {
            return block_35(state, tracer, fn_state);
        };
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_35_0: const #20u : u8
        let s_35_0: u8 = 20;
        // C s_35_1: cast zx s_35_0 -> bv
        let s_35_1: Bits = Bits::new(s_35_0 as u128, 8u16);
        // C s_35_2: cast zx s_35_1 -> i
        let s_35_2: i128 = (s_35_1.value() as i128);
        // C s_35_3: cast reint s_35_2 -> i64
        let s_35_3: i64 = (s_35_2 as i64);
        // C s_35_4: cast zx s_35_3 -> i
        let s_35_4: i128 = (i128::try_from(s_35_3).unwrap());
        // D s_35_5: read-var except:struct
        let s_35_5: ProductTypeb7f99f96751e17c4 = fn_state.except;
        // D s_35_6: read-var preferred_exception_return:u32
        let s_35_6: u32 = fn_state.preferred_exception_return;
        // D s_35_7: call AArch32_EnterHypMode(s_35_5, s_35_6, s_35_4)
        let s_35_7: () = AArch32_EnterHypMode(state, tracer, s_35_5, s_35_6, s_35_4);
        // N s_35_8: return
        return;
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_36_0: read-var vect_offset:i64
        let s_36_0: i64 = fn_state.vect_offset;
        // D s_36_1: cast zx s_36_0 -> i
        let s_36_1: i128 = (i128::try_from(s_36_0).unwrap());
        // D s_36_2: read-var except:struct
        let s_36_2: ProductTypeb7f99f96751e17c4 = fn_state.except;
        // D s_36_3: read-var preferred_exception_return:u32
        let s_36_3: u32 = fn_state.preferred_exception_return;
        // D s_36_4: call AArch32_EnterHypMode(s_36_2, s_36_3, s_36_1)
        let s_36_4: () = AArch32_EnterHypMode(state, tracer, s_36_2, s_36_3, s_36_1);
        // N s_36_5: return
        return;
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_37_0: read-var target_el:u8
        let s_37_0: u8 = fn_state.target_el;
        // D s_37_1: cast zx s_37_0 -> bv
        let s_37_1: Bits = Bits::new(s_37_0 as u128, 2u16);
        // C s_37_2: const #440u : u32
        let s_37_2: u32 = 440;
        // D s_37_3: read-reg s_37_2:u8
        let s_37_3: u8 = {
            let value = state.read_register::<u8>(s_37_2 as isize);
            tracer.read_register(s_37_2 as isize, value);
            value
        };
        // D s_37_4: cast zx s_37_3 -> bv
        let s_37_4: Bits = Bits::new(s_37_3 as u128, 2u16);
        // D s_37_5: cmp-eq s_37_1 s_37_4
        let s_37_5: bool = ((s_37_1) == (s_37_4));
        // D s_37_6: not s_37_5
        let s_37_6: bool = !s_37_5;
        // N s_37_7: branch s_37_6 b39 b38
        if s_37_6 {
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
        // D s_38_0: read-var route_to_monitor:u8
        let s_38_0: bool = fn_state.route_to_monitor;
        // D s_38_1: read-var fault:struct
        let s_38_1: ProductType1d757adad216cdef = fn_state.fault;
        // D s_38_2: read-var vaddress:u32
        let s_38_2: u32 = fn_state.vaddress;
        // D s_38_3: call AArch32_ReportDataAbort(s_38_0, s_38_1, s_38_2)
        let s_38_3: () = AArch32_ReportDataAbort(state, tracer, s_38_0, s_38_1, s_38_2);
        // C s_38_4: const #8s : i64
        let s_38_4: i64 = 8;
        // C s_38_5: cast zx s_38_4 -> i
        let s_38_5: i128 = (i128::try_from(s_38_4).unwrap());
        // D s_38_6: read-var vect_offset:i64
        let s_38_6: i64 = fn_state.vect_offset;
        // D s_38_7: cast zx s_38_6 -> i
        let s_38_7: i128 = (i128::try_from(s_38_6).unwrap());
        // C s_38_8: const #392u : u32
        let s_38_8: u32 = 392;
        // D s_38_9: read-reg s_38_8:u8
        let s_38_9: u8 = {
            let value = state.read_register::<u8>(s_38_8 as isize);
            tracer.read_register(s_38_8 as isize, value);
            value
        };
        // D s_38_10: read-var preferred_exception_return:u32
        let s_38_10: u32 = fn_state.preferred_exception_return;
        // D s_38_11: call AArch32_EnterMode(s_38_9, s_38_10, s_38_5, s_38_7)
        let s_38_11: () = AArch32_EnterMode(
            state,
            tracer,
            s_38_9,
            s_38_10,
            s_38_5,
            s_38_7,
        );
        // N s_38_12: return
        return;
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_39_0: const #() : ()
        let s_39_0: () = ();
        // S s_39_1: call Unreachable(s_39_0)
        let s_39_1: () = Unreachable(state, tracer, s_39_0);
        // N s_39_2: return
        return;
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_40_0: const #() : ()
        let s_40_0: () = ();
        // S s_40_1: call ClearPendingPhysicalSError(s_40_0)
        let s_40_1: () = ClearPendingPhysicalSError(state, tracer, s_40_0);
        // N s_40_2: jump b31
        return block_31(state, tracer, fn_state);
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_41_0: const #432u : u32
        let s_41_0: u32 = 432;
        // D s_41_1: read-reg s_41_0:u8
        let s_41_1: u8 = {
            let value = state.read_register::<u8>(s_41_0 as isize);
            tracer.read_register(s_41_0 as isize, value);
            value
        };
        // D s_41_2: write-var target_el <= s_41_1
        fn_state.target_el = s_41_1;
        // N s_41_3: jump b29
        return block_29(state, tracer, fn_state);
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_42_0: const #1u : u8
        let s_42_0: bool = true;
        // D s_42_1: write-var gs#31963 <= s_42_0
        fn_state.gs_31963 = s_42_0;
        // N s_42_2: jump b27
        return block_27(state, tracer, fn_state);
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_43_0: const #424u : u32
        let s_43_0: u32 = 424;
        // D s_43_1: read-reg s_43_0:u8
        let s_43_1: u8 = {
            let value = state.read_register::<u8>(s_43_0 as isize);
            tracer.read_register(s_43_0 as isize, value);
            value
        };
        // D s_43_2: write-var target_el <= s_43_1
        fn_state.target_el = s_43_1;
        // N s_43_3: jump b29
        return block_29(state, tracer, fn_state);
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_44_0: const #() : ()
        let s_44_0: () = ();
        // S s_44_1: call HCR_read(s_44_0)
        let s_44_1: ProductType700c18a878c5601b = HCR_read(state, tracer, s_44_0);
        // S s_44_2: call _get_HCR_Type_TGE(s_44_1)
        let s_44_2: bool = u_get_HCR_Type_TGE(state, tracer, s_44_1);
        // S s_44_3: cast zx s_44_2 -> bv
        let s_44_3: Bits = Bits::new(s_44_2 as u128, 1u16);
        // C s_44_4: const #1u : u8
        let s_44_4: bool = true;
        // C s_44_5: cast zx s_44_4 -> bv
        let s_44_5: Bits = Bits::new(s_44_4 as u128, 1u16);
        // S s_44_6: cmp-eq s_44_3 s_44_5
        let s_44_6: bool = ((s_44_3) == (s_44_5));
        // N s_44_7: branch s_44_6 b47 b45
        if s_44_6 {
            return block_47(state, tracer, fn_state);
        } else {
            return block_45(state, tracer, fn_state);
        };
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_45_0: const #() : ()
        let s_45_0: () = ();
        // S s_45_1: call HCR_read(s_45_0)
        let s_45_1: ProductType700c18a878c5601b = HCR_read(state, tracer, s_45_0);
        // S s_45_2: call _get_HCR_Type_AMO(s_45_1)
        let s_45_2: bool = u_get_HCR_Type_AMO(state, tracer, s_45_1);
        // S s_45_3: cast zx s_45_2 -> bv
        let s_45_3: Bits = Bits::new(s_45_2 as u128, 1u16);
        // C s_45_4: const #1u : u8
        let s_45_4: bool = true;
        // C s_45_5: cast zx s_45_4 -> bv
        let s_45_5: Bits = Bits::new(s_45_4 as u128, 1u16);
        // S s_45_6: cmp-eq s_45_3 s_45_5
        let s_45_6: bool = ((s_45_3) == (s_45_5));
        // D s_45_7: write-var gs#31956 <= s_45_6
        fn_state.gs_31956 = s_45_6;
        // N s_45_8: jump b46
        return block_46(state, tracer, fn_state);
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_46_0: read-var gs#31956:u8
        let s_46_0: bool = fn_state.gs_31956;
        // D s_46_1: write-var gs#31957 <= s_46_0
        fn_state.gs_31957 = s_46_0;
        // N s_46_2: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_47_0: const #1u : u8
        let s_47_0: bool = true;
        // D s_47_1: write-var gs#31956 <= s_47_0
        fn_state.gs_31956 = s_47_0;
        // N s_47_2: jump b46
        return block_46(state, tracer, fn_state);
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_48_0: const #() : ()
        let s_48_0: () = ();
        // S s_48_1: call EL2Enabled(s_48_0)
        let s_48_1: bool = EL2Enabled(state, tracer, s_48_0);
        // D s_48_2: write-var gs#31955 <= s_48_1
        fn_state.gs_31955 = s_48_1;
        // N s_48_3: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_49_0: const #1u : u8
        let s_49_0: bool = true;
        // D s_49_1: write-var gs#31954 <= s_49_0
        fn_state.gs_31954 = s_49_0;
        // N s_49_2: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_50<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_50_0: const #20920u : u32
        let s_50_0: u32 = 20920;
        // D s_50_1: read-reg s_50_0:struct
        let s_50_1: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_50_0 as isize);
            tracer.read_register(s_50_0 as isize, value);
            value
        };
        // D s_50_2: call _get_SCR_Type_EA(s_50_1)
        let s_50_2: bool = u_get_SCR_Type_EA(state, tracer, s_50_1);
        // D s_50_3: cast zx s_50_2 -> bv
        let s_50_3: Bits = Bits::new(s_50_2 as u128, 1u16);
        // C s_50_4: const #1u : u8
        let s_50_4: bool = true;
        // C s_50_5: cast zx s_50_4 -> bv
        let s_50_5: Bits = Bits::new(s_50_4 as u128, 1u16);
        // D s_50_6: cmp-eq s_50_3 s_50_5
        let s_50_6: bool = ((s_50_3) == (s_50_5));
        // D s_50_7: write-var gs#31953 <= s_50_6
        fn_state.gs_31953 = s_50_6;
        // N s_50_8: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_51<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_51_0: read-var implicit_esb:u8
        let s_51_0: bool = fn_state.implicit_esb;
        // D s_51_1: call AArch64_TakePhysicalSErrorException(s_51_0)
        let s_51_1: () = AArch64_TakePhysicalSErrorException(state, tracer, s_51_0);
        // N s_51_2: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_52<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_52_0: const #() : ()
        let s_52_0: () = ();
        // S s_52_1: call EffectiveEA(s_52_0)
        let s_52_1: bool = EffectiveEA(state, tracer, s_52_0);
        // S s_52_2: cast zx s_52_1 -> bv
        let s_52_2: Bits = Bits::new(s_52_1 as u128, 1u16);
        // C s_52_3: const #1u : u8
        let s_52_3: bool = true;
        // C s_52_4: cast zx s_52_3 -> bv
        let s_52_4: Bits = Bits::new(s_52_3 as u128, 1u16);
        // S s_52_5: cmp-eq s_52_2 s_52_4
        let s_52_5: bool = ((s_52_2) == (s_52_4));
        // D s_52_6: write-var route_to_aarch64 <= s_52_5
        fn_state.route_to_aarch64 = s_52_5;
        // N s_52_7: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_53<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_53_0: const #424u : u32
        let s_53_0: u32 = 424;
        // D s_53_1: read-reg s_53_0:u8
        let s_53_1: u8 = {
            let value = state.read_register::<u8>(s_53_0 as isize);
            tracer.read_register(s_53_0 as isize, value);
            value
        };
        // D s_53_2: call ELUsingAArch32(s_53_1)
        let s_53_2: bool = ELUsingAArch32(state, tracer, s_53_1);
        // D s_53_3: not s_53_2
        let s_53_3: bool = !s_53_2;
        // D s_53_4: write-var gs#31947 <= s_53_3
        fn_state.gs_31947 = s_53_3;
        // N s_53_5: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_54<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_54_0: const #424u : u32
        let s_54_0: u32 = 424;
        // D s_54_1: read-reg s_54_0:u8
        let s_54_1: u8 = {
            let value = state.read_register::<u8>(s_54_0 as isize);
            tracer.read_register(s_54_0 as isize, value);
            value
        };
        // C s_54_2: const #2u : u8
        let s_54_2: u8 = 2;
        // D s_54_3: cmp-lt s_54_1 s_54_2
        let s_54_3: bool = ((s_54_1) < (s_54_2));
        // D s_54_4: write-var gs#31946 <= s_54_3
        fn_state.gs_31946 = s_54_3;
        // N s_54_5: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_55<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_55_0: const #102552u : u32
        let s_55_0: u32 = 102552;
        // D s_55_1: read-reg s_55_0:struct
        let s_55_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_55_0 as isize);
            tracer.read_register(s_55_0 as isize, value);
            value
        };
        // D s_55_2: call _get_HCR_EL2_Type_TGE(s_55_1)
        let s_55_2: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_55_1);
        // D s_55_3: cast zx s_55_2 -> bv
        let s_55_3: Bits = Bits::new(s_55_2 as u128, 1u16);
        // C s_55_4: const #1u : u8
        let s_55_4: bool = true;
        // C s_55_5: cast zx s_55_4 -> bv
        let s_55_5: Bits = Bits::new(s_55_4 as u128, 1u16);
        // D s_55_6: cmp-eq s_55_3 s_55_5
        let s_55_6: bool = ((s_55_3) == (s_55_5));
        // N s_55_7: branch s_55_6 b61 b56
        if s_55_6 {
            return block_61(state, tracer, fn_state);
        } else {
            return block_56(state, tracer, fn_state);
        };
    }
    fn block_56<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_56_0: const #() : ()
        let s_56_0: () = ();
        // S s_56_1: call IsInHost(s_56_0)
        let s_56_1: bool = IsInHost(state, tracer, s_56_0);
        // S s_56_2: not s_56_1
        let s_56_2: bool = !s_56_1;
        // N s_56_3: branch s_56_2 b60 b57
        if s_56_2 {
            return block_60(state, tracer, fn_state);
        } else {
            return block_57(state, tracer, fn_state);
        };
    }
    fn block_57<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_57_0: const #0u : u8
        let s_57_0: bool = false;
        // D s_57_1: write-var gs#31951 <= s_57_0
        fn_state.gs_31951 = s_57_0;
        // N s_57_2: jump b58
        return block_58(state, tracer, fn_state);
    }
    fn block_58<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_58_0: read-var gs#31951:u8
        let s_58_0: bool = fn_state.gs_31951;
        // D s_58_1: write-var gs#31952 <= s_58_0
        fn_state.gs_31952 = s_58_0;
        // N s_58_2: jump b59
        return block_59(state, tracer, fn_state);
    }
    fn block_59<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_59_0: read-var gs#31952:u8
        let s_59_0: bool = fn_state.gs_31952;
        // D s_59_1: write-var route_to_aarch64 <= s_59_0
        fn_state.route_to_aarch64 = s_59_0;
        // N s_59_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_60<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_60_0: const #102552u : u32
        let s_60_0: u32 = 102552;
        // D s_60_1: read-reg s_60_0:struct
        let s_60_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_60_0 as isize);
            tracer.read_register(s_60_0 as isize, value);
            value
        };
        // D s_60_2: call _get_HCR_EL2_Type_AMO(s_60_1)
        let s_60_2: bool = u_get_HCR_EL2_Type_AMO(state, tracer, s_60_1);
        // D s_60_3: cast zx s_60_2 -> bv
        let s_60_3: Bits = Bits::new(s_60_2 as u128, 1u16);
        // C s_60_4: const #1u : u8
        let s_60_4: bool = true;
        // C s_60_5: cast zx s_60_4 -> bv
        let s_60_5: Bits = Bits::new(s_60_4 as u128, 1u16);
        // D s_60_6: cmp-eq s_60_3 s_60_5
        let s_60_6: bool = ((s_60_3) == (s_60_5));
        // D s_60_7: write-var gs#31951 <= s_60_6
        fn_state.gs_31951 = s_60_6;
        // N s_60_8: jump b58
        return block_58(state, tracer, fn_state);
    }
    fn block_61<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_61_0: const #1u : u8
        let s_61_0: bool = true;
        // D s_61_1: write-var gs#31952 <= s_61_0
        fn_state.gs_31952 = s_61_0;
        // N s_61_2: jump b59
        return block_59(state, tracer, fn_state);
    }
    fn block_62<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_62_0: const #432u : u32
        let s_62_0: u32 = 432;
        // D s_62_1: read-reg s_62_0:u8
        let s_62_1: u8 = {
            let value = state.read_register::<u8>(s_62_0 as isize);
            tracer.read_register(s_62_0 as isize, value);
            value
        };
        // D s_62_2: call ELUsingAArch32(s_62_1)
        let s_62_2: bool = ELUsingAArch32(state, tracer, s_62_1);
        // D s_62_3: not s_62_2
        let s_62_3: bool = !s_62_2;
        // D s_62_4: write-var gs#31945 <= s_62_3
        fn_state.gs_31945 = s_62_3;
        // N s_62_5: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_63<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_63_0: const #() : ()
        let s_63_0: () = ();
        // S s_63_1: call EL2Enabled(s_63_0)
        let s_63_1: bool = EL2Enabled(state, tracer, s_63_0);
        // D s_63_2: write-var gs#31944 <= s_63_1
        fn_state.gs_31944 = s_63_1;
        // N s_63_3: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_64<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_64_0: const #440u : u32
        let s_64_0: u32 = 440;
        // D s_64_1: read-reg s_64_0:u8
        let s_64_1: u8 = {
            let value = state.read_register::<u8>(s_64_0 as isize);
            tracer.read_register(s_64_0 as isize, value);
            value
        };
        // D s_64_2: call ELUsingAArch32(s_64_1)
        let s_64_2: bool = ELUsingAArch32(state, tracer, s_64_1);
        // D s_64_3: not s_64_2
        let s_64_3: bool = !s_64_2;
        // D s_64_4: write-var gs#31943 <= s_64_3
        fn_state.gs_31943 = s_64_3;
        // N s_64_5: jump b2
        return block_2(state, tracer, fn_state);
    }
}
