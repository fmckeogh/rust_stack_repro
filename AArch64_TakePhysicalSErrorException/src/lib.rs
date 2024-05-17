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
use ThisInstrAddr::*;
use IsInHost::*;
use ClearPendingPhysicalSError::*;
use AArch64_TakeException::*;
use AArch64_PhysicalSErrorSyndrome::*;
use ExceptionSyndrome::*;
use IsSErrorEdgeTriggered::*;
use u_get_SCR_EL3_Type_EA::*;
use u_get_HCR_EL2_Type_TGE::*;
use u_get_HCR_EL2_Type_AMO::*;
use EL2Enabled::*;
use common::*;
pub fn AArch64_TakePhysicalSErrorException<T: Tracer>(
    state: &mut State,
    tracer: &T,
    implicit_esb: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        route_to_el2: bool,
        gs_25206: bool,
        gs_25201: bool,
        except: ProductTypeb7f99f96751e17c4,
        gs_25203: bool,
        preferred_exception_return: u64,
        route_to_el3: bool,
        vect_offset: i64,
        gs_25211: bool,
        gs_25209: bool,
        gs_25204: bool,
        gs_25202: bool,
        syndrome: u32,
        gs_25205: bool,
        target_el: u8,
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
        // N s_0_4: branch s_0_3 b32 b1
        if s_0_3 {
            return block_32(state, tracer, fn_state);
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
        // D s_1_1: write-var gs#25201 <= s_1_0
        fn_state.gs_25201 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var gs#25201:u8
        let s_2_0: bool = fn_state.gs_25201;
        // D s_2_1: write-var route_to_el3 <= s_2_0
        fn_state.route_to_el3 = s_2_0;
        // C s_2_2: const #16975u : u32
        let s_2_2: u32 = 16975;
        // D s_2_3: read-reg s_2_2:u8
        let s_2_3: u8 = {
            let value = state.read_register::<u8>(s_2_2 as isize);
            tracer.read_register(s_2_2 as isize, value);
            value
        };
        // D s_2_4: cast zx s_2_3 -> bv
        let s_2_4: Bits = Bits::new(s_2_3 as u128, 2u16);
        // C s_2_5: const #448u : u32
        let s_2_5: u32 = 448;
        // D s_2_6: read-reg s_2_5:u8
        let s_2_6: u8 = {
            let value = state.read_register::<u8>(s_2_5 as isize);
            tracer.read_register(s_2_5 as isize, value);
            value
        };
        // D s_2_7: cast zx s_2_6 -> bv
        let s_2_7: Bits = Bits::new(s_2_6 as u128, 2u16);
        // D s_2_8: cmp-eq s_2_4 s_2_7
        let s_2_8: bool = ((s_2_4) == (s_2_7));
        // N s_2_9: branch s_2_8 b31 b3
        if s_2_8 {
            return block_31(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #16975u : u32
        let s_3_0: u32 = 16975;
        // D s_3_1: read-reg s_3_0:u8
        let s_3_1: u8 = {
            let value = state.read_register::<u8>(s_3_0 as isize);
            tracer.read_register(s_3_0 as isize, value);
            value
        };
        // D s_3_2: cast zx s_3_1 -> bv
        let s_3_2: Bits = Bits::new(s_3_1 as u128, 2u16);
        // C s_3_3: const #440u : u32
        let s_3_3: u32 = 440;
        // D s_3_4: read-reg s_3_3:u8
        let s_3_4: u8 = {
            let value = state.read_register::<u8>(s_3_3 as isize);
            tracer.read_register(s_3_3 as isize, value);
            value
        };
        // D s_3_5: cast zx s_3_4 -> bv
        let s_3_5: Bits = Bits::new(s_3_4 as u128, 2u16);
        // D s_3_6: cmp-eq s_3_2 s_3_5
        let s_3_6: bool = ((s_3_2) == (s_3_5));
        // D s_3_7: write-var gs#25202 <= s_3_6
        fn_state.gs_25202 = s_3_6;
        // N s_3_8: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var gs#25202:u8
        let s_4_0: bool = fn_state.gs_25202;
        // N s_4_1: branch s_4_0 b30 b5
        if s_4_0 {
            return block_30(state, tracer, fn_state);
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
        // D s_5_1: write-var gs#25203 <= s_5_0
        fn_state.gs_25203 = s_5_0;
        // N s_5_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var gs#25203:u8
        let s_6_0: bool = fn_state.gs_25203;
        // N s_6_1: branch s_6_0 b23 b7
        if s_6_0 {
            return block_23(state, tracer, fn_state);
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
        // D s_7_1: write-var gs#25206 <= s_7_0
        fn_state.gs_25206 = s_7_0;
        // N s_7_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var gs#25206:u8
        let s_8_0: bool = fn_state.gs_25206;
        // D s_8_1: write-var route_to_el2 <= s_8_0
        fn_state.route_to_el2 = s_8_0;
        // C s_8_2: const #64s : i64
        let s_8_2: i64 = 64;
        // C s_8_3: cast zx s_8_2 -> i
        let s_8_3: i128 = (i128::try_from(s_8_2).unwrap());
        // S s_8_4: call ThisInstrAddr(s_8_3)
        let s_8_4: Bits = ThisInstrAddr(state, tracer, s_8_3);
        // S s_8_5: cast reint s_8_4 -> u64
        let s_8_5: u64 = (s_8_4.value() as u64);
        // D s_8_6: write-var preferred_exception_return <= s_8_5
        fn_state.preferred_exception_return = s_8_5;
        // C s_8_7: const #384u : u12
        let s_8_7: u16 = 384;
        // C s_8_8: cast zx s_8_7 -> bv
        let s_8_8: Bits = Bits::new(s_8_7 as u128, 12u16);
        // C s_8_9: cast zx s_8_8 -> i
        let s_8_9: i128 = (s_8_8.value() as i128);
        // C s_8_10: cast reint s_8_9 -> i64
        let s_8_10: i64 = (s_8_9 as i64);
        // D s_8_11: write-var vect_offset <= s_8_10
        fn_state.vect_offset = s_8_10;
        // C s_8_12: const #16975u : u32
        let s_8_12: u32 = 16975;
        // D s_8_13: read-reg s_8_12:u8
        let s_8_13: u8 = {
            let value = state.read_register::<u8>(s_8_12 as isize);
            tracer.read_register(s_8_12 as isize, value);
            value
        };
        // D s_8_14: cast zx s_8_13 -> bv
        let s_8_14: Bits = Bits::new(s_8_13 as u128, 2u16);
        // C s_8_15: const #424u : u32
        let s_8_15: u32 = 424;
        // D s_8_16: read-reg s_8_15:u8
        let s_8_16: u8 = {
            let value = state.read_register::<u8>(s_8_15 as isize);
            tracer.read_register(s_8_15 as isize, value);
            value
        };
        // D s_8_17: cast zx s_8_16 -> bv
        let s_8_17: Bits = Bits::new(s_8_16 as u128, 2u16);
        // D s_8_18: cmp-eq s_8_14 s_8_17
        let s_8_18: bool = ((s_8_14) == (s_8_17));
        // N s_8_19: branch s_8_18 b22 b9
        if s_8_18 {
            return block_22(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var route_to_el3:u8
        let s_9_0: bool = fn_state.route_to_el3;
        // D s_9_1: write-var gs#25209 <= s_9_0
        fn_state.gs_25209 = s_9_0;
        // N s_9_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var gs#25209:u8
        let s_10_0: bool = fn_state.gs_25209;
        // N s_10_1: branch s_10_0 b21 b11
        if s_10_0 {
            return block_21(state, tracer, fn_state);
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
        // N s_11_7: branch s_11_6 b20 b12
        if s_11_6 {
            return block_20(state, tracer, fn_state);
        } else {
            return block_12(state, tracer, fn_state);
        };
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var route_to_el2:u8
        let s_12_0: bool = fn_state.route_to_el2;
        // D s_12_1: write-var gs#25211 <= s_12_0
        fn_state.gs_25211 = s_12_0;
        // N s_12_2: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var gs#25211:u8
        let s_13_0: bool = fn_state.gs_25211;
        // N s_13_1: branch s_13_0 b19 b14
        if s_13_0 {
            return block_19(state, tracer, fn_state);
        } else {
            return block_14(state, tracer, fn_state);
        };
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_14_0: const #440u : u32
        let s_14_0: u32 = 440;
        // D s_14_1: read-reg s_14_0:u8
        let s_14_1: u8 = {
            let value = state.read_register::<u8>(s_14_0 as isize);
            tracer.read_register(s_14_0 as isize, value);
            value
        };
        // D s_14_2: write-var target_el <= s_14_1
        fn_state.target_el = s_14_1;
        // N s_14_3: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_15_0: const #24u : u32
        let s_15_0: u32 = 24;
        // S s_15_1: call ExceptionSyndrome(s_15_0)
        let s_15_1: ProductTypeb7f99f96751e17c4 = ExceptionSyndrome(
            state,
            tracer,
            s_15_0,
        );
        // D s_15_2: write-var except <= s_15_1
        fn_state.except = s_15_1;
        // D s_15_3: read-var implicit_esb:u8
        let s_15_3: bool = fn_state.implicit_esb;
        // D s_15_4: call AArch64_PhysicalSErrorSyndrome(s_15_3)
        let s_15_4: u32 = AArch64_PhysicalSErrorSyndrome(state, tracer, s_15_3);
        // D s_15_5: write-var syndrome <= s_15_4
        fn_state.syndrome = s_15_4;
        // C s_15_6: const #() : ()
        let s_15_6: () = ();
        // S s_15_7: call IsSErrorEdgeTriggered(s_15_6)
        let s_15_7: bool = IsSErrorEdgeTriggered(state, tracer, s_15_6);
        // N s_15_8: branch s_15_7 b18 b16
        if s_15_7 {
            return block_18(state, tracer, fn_state);
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
        // D s_17_0: read-var syndrome:u25
        let s_17_0: u32 = fn_state.syndrome;
        // D s_17_1: write-var except.6 <= s_17_0
        fn_state.except._6 = s_17_0;
        // D s_17_2: read-var vect_offset:i64
        let s_17_2: i64 = fn_state.vect_offset;
        // D s_17_3: cast zx s_17_2 -> i
        let s_17_3: i128 = (i128::try_from(s_17_2).unwrap());
        // D s_17_4: read-var target_el:u8
        let s_17_4: u8 = fn_state.target_el;
        // D s_17_5: read-var except:struct
        let s_17_5: ProductTypeb7f99f96751e17c4 = fn_state.except;
        // D s_17_6: read-var preferred_exception_return:u64
        let s_17_6: u64 = fn_state.preferred_exception_return;
        // D s_17_7: call AArch64_TakeException(s_17_4, s_17_5, s_17_6, s_17_3)
        let s_17_7: () = AArch64_TakeException(
            state,
            tracer,
            s_17_4,
            s_17_5,
            s_17_6,
            s_17_3,
        );
        // N s_17_8: return
        return;
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_18_0: const #() : ()
        let s_18_0: () = ();
        // S s_18_1: call ClearPendingPhysicalSError(s_18_0)
        let s_18_1: () = ClearPendingPhysicalSError(state, tracer, s_18_0);
        // N s_18_2: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_19_0: const #432u : u32
        let s_19_0: u32 = 432;
        // D s_19_1: read-reg s_19_0:u8
        let s_19_1: u8 = {
            let value = state.read_register::<u8>(s_19_0 as isize);
            tracer.read_register(s_19_0 as isize, value);
            value
        };
        // D s_19_2: write-var target_el <= s_19_1
        fn_state.target_el = s_19_1;
        // N s_19_3: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_20_0: const #1u : u8
        let s_20_0: bool = true;
        // D s_20_1: write-var gs#25211 <= s_20_0
        fn_state.gs_25211 = s_20_0;
        // N s_20_2: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_21_0: const #424u : u32
        let s_21_0: u32 = 424;
        // D s_21_1: read-reg s_21_0:u8
        let s_21_1: u8 = {
            let value = state.read_register::<u8>(s_21_0 as isize);
            tracer.read_register(s_21_0 as isize, value);
            value
        };
        // D s_21_2: write-var target_el <= s_21_1
        fn_state.target_el = s_21_1;
        // N s_21_3: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_22_0: const #1u : u8
        let s_22_0: bool = true;
        // D s_22_1: write-var gs#25209 <= s_22_0
        fn_state.gs_25209 = s_22_0;
        // N s_22_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_23_0: const #102552u : u32
        let s_23_0: u32 = 102552;
        // D s_23_1: read-reg s_23_0:struct
        let s_23_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_23_0 as isize);
            tracer.read_register(s_23_0 as isize, value);
            value
        };
        // D s_23_2: call _get_HCR_EL2_Type_TGE(s_23_1)
        let s_23_2: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_23_1);
        // D s_23_3: cast zx s_23_2 -> bv
        let s_23_3: Bits = Bits::new(s_23_2 as u128, 1u16);
        // C s_23_4: const #1u : u8
        let s_23_4: bool = true;
        // C s_23_5: cast zx s_23_4 -> bv
        let s_23_5: Bits = Bits::new(s_23_4 as u128, 1u16);
        // D s_23_6: cmp-eq s_23_3 s_23_5
        let s_23_6: bool = ((s_23_3) == (s_23_5));
        // N s_23_7: branch s_23_6 b29 b24
        if s_23_6 {
            return block_29(state, tracer, fn_state);
        } else {
            return block_24(state, tracer, fn_state);
        };
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_24_0: const #() : ()
        let s_24_0: () = ();
        // S s_24_1: call IsInHost(s_24_0)
        let s_24_1: bool = IsInHost(state, tracer, s_24_0);
        // S s_24_2: not s_24_1
        let s_24_2: bool = !s_24_1;
        // N s_24_3: branch s_24_2 b28 b25
        if s_24_2 {
            return block_28(state, tracer, fn_state);
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
        // D s_25_1: write-var gs#25204 <= s_25_0
        fn_state.gs_25204 = s_25_0;
        // N s_25_2: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_26_0: read-var gs#25204:u8
        let s_26_0: bool = fn_state.gs_25204;
        // D s_26_1: write-var gs#25205 <= s_26_0
        fn_state.gs_25205 = s_26_0;
        // N s_26_2: jump b27
        return block_27(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_27_0: read-var gs#25205:u8
        let s_27_0: bool = fn_state.gs_25205;
        // D s_27_1: write-var gs#25206 <= s_27_0
        fn_state.gs_25206 = s_27_0;
        // N s_27_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_28_0: const #102552u : u32
        let s_28_0: u32 = 102552;
        // D s_28_1: read-reg s_28_0:struct
        let s_28_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_28_0 as isize);
            tracer.read_register(s_28_0 as isize, value);
            value
        };
        // D s_28_2: call _get_HCR_EL2_Type_AMO(s_28_1)
        let s_28_2: bool = u_get_HCR_EL2_Type_AMO(state, tracer, s_28_1);
        // D s_28_3: cast zx s_28_2 -> bv
        let s_28_3: Bits = Bits::new(s_28_2 as u128, 1u16);
        // C s_28_4: const #1u : u8
        let s_28_4: bool = true;
        // C s_28_5: cast zx s_28_4 -> bv
        let s_28_5: Bits = Bits::new(s_28_4 as u128, 1u16);
        // D s_28_6: cmp-eq s_28_3 s_28_5
        let s_28_6: bool = ((s_28_3) == (s_28_5));
        // D s_28_7: write-var gs#25204 <= s_28_6
        fn_state.gs_25204 = s_28_6;
        // N s_28_8: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_29_0: const #1u : u8
        let s_29_0: bool = true;
        // D s_29_1: write-var gs#25205 <= s_29_0
        fn_state.gs_25205 = s_29_0;
        // N s_29_2: jump b27
        return block_27(state, tracer, fn_state);
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_30_0: const #() : ()
        let s_30_0: () = ();
        // S s_30_1: call EL2Enabled(s_30_0)
        let s_30_1: bool = EL2Enabled(state, tracer, s_30_0);
        // D s_30_2: write-var gs#25203 <= s_30_1
        fn_state.gs_25203 = s_30_1;
        // N s_30_3: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_31_0: const #1u : u8
        let s_31_0: bool = true;
        // D s_31_1: write-var gs#25202 <= s_31_0
        fn_state.gs_25202 = s_31_0;
        // N s_31_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_32_0: const #90704u : u32
        let s_32_0: u32 = 90704;
        // D s_32_1: read-reg s_32_0:struct
        let s_32_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_32_0 as isize);
            tracer.read_register(s_32_0 as isize, value);
            value
        };
        // D s_32_2: call _get_SCR_EL3_Type_EA(s_32_1)
        let s_32_2: bool = u_get_SCR_EL3_Type_EA(state, tracer, s_32_1);
        // D s_32_3: cast zx s_32_2 -> bv
        let s_32_3: Bits = Bits::new(s_32_2 as u128, 1u16);
        // C s_32_4: const #1u : u8
        let s_32_4: bool = true;
        // C s_32_5: cast zx s_32_4 -> bv
        let s_32_5: Bits = Bits::new(s_32_4 as u128, 1u16);
        // D s_32_6: cmp-eq s_32_3 s_32_5
        let s_32_6: bool = ((s_32_3) == (s_32_5));
        // D s_32_7: write-var gs#25201 <= s_32_6
        fn_state.gs_25201 = s_32_6;
        // N s_32_8: jump b2
        return block_2(state, tracer, fn_state);
    }
}
