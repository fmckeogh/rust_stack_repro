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
use EL2Enabled::*;
use u_get_SCR_EL3_Type_IRQ::*;
use u_get_HCR_EL2_Type_TGE::*;
use AArch64_TakeException::*;
use ExceptionSyndrome::*;
use u_get_HCR_EL2_Type_IMO::*;
use common::*;
pub fn AArch64_TakePhysicalIRQException<T: Tracer>(
    state: &mut State,
    tracer: &T,
    gs_25236: (),
) -> () {
    #[derive(Default)]
    struct FunctionState {
        route_to_el2: bool,
        gs_25240: bool,
        except: ProductTypeb7f99f96751e17c4,
        gs_25239: bool,
        gs_25246: bool,
        preferred_exception_return: u64,
        route_to_el3: bool,
        gs_25237: bool,
        gs_25245: bool,
        vect_offset: i64,
        gs_25238: bool,
        gs_25241: bool,
        gs_25236: (),
    }
    let fn_state = FunctionState {
        gs_25236,
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
        // N s_0_4: branch s_0_3 b25 b1
        if s_0_3 {
            return block_25(state, tracer, fn_state);
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
        // D s_1_1: write-var gs#25237 <= s_1_0
        fn_state.gs_25237 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var gs#25237:u8
        let s_2_0: bool = fn_state.gs_25237;
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
        // N s_2_9: branch s_2_8 b24 b3
        if s_2_8 {
            return block_24(state, tracer, fn_state);
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
        // D s_3_7: write-var gs#25238 <= s_3_6
        fn_state.gs_25238 = s_3_6;
        // N s_3_8: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var gs#25238:u8
        let s_4_0: bool = fn_state.gs_25238;
        // N s_4_1: branch s_4_0 b23 b5
        if s_4_0 {
            return block_23(state, tracer, fn_state);
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
        // D s_5_1: write-var gs#25239 <= s_5_0
        fn_state.gs_25239 = s_5_0;
        // N s_5_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var gs#25239:u8
        let s_6_0: bool = fn_state.gs_25239;
        // N s_6_1: branch s_6_0 b19 b7
        if s_6_0 {
            return block_19(state, tracer, fn_state);
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
        // D s_7_1: write-var gs#25241 <= s_7_0
        fn_state.gs_25241 = s_7_0;
        // N s_7_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var gs#25241:u8
        let s_8_0: bool = fn_state.gs_25241;
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
        // C s_8_7: const #128u : u8
        let s_8_7: u8 = 128;
        // C s_8_8: cast zx s_8_7 -> bv
        let s_8_8: Bits = Bits::new(s_8_7 as u128, 8u16);
        // C s_8_9: cast zx s_8_8 -> i
        let s_8_9: i128 = (s_8_8.value() as i128);
        // C s_8_10: cast reint s_8_9 -> i64
        let s_8_10: i64 = (s_8_9 as i64);
        // D s_8_11: write-var vect_offset <= s_8_10
        fn_state.vect_offset = s_8_10;
        // C s_8_12: const #31u : u32
        let s_8_12: u32 = 31;
        // S s_8_13: call ExceptionSyndrome(s_8_12)
        let s_8_13: ProductTypeb7f99f96751e17c4 = ExceptionSyndrome(
            state,
            tracer,
            s_8_12,
        );
        // D s_8_14: write-var except <= s_8_13
        fn_state.except = s_8_13;
        // D s_8_15: read-var route_to_el3:u8
        let s_8_15: bool = fn_state.route_to_el3;
        // N s_8_16: branch s_8_15 b18 b9
        if s_8_15 {
            return block_18(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_9_0: const #16975u : u32
        let s_9_0: u32 = 16975;
        // D s_9_1: read-reg s_9_0:u8
        let s_9_1: u8 = {
            let value = state.read_register::<u8>(s_9_0 as isize);
            tracer.read_register(s_9_0 as isize, value);
            value
        };
        // D s_9_2: cast zx s_9_1 -> bv
        let s_9_2: Bits = Bits::new(s_9_1 as u128, 2u16);
        // C s_9_3: const #432u : u32
        let s_9_3: u32 = 432;
        // D s_9_4: read-reg s_9_3:u8
        let s_9_4: u8 = {
            let value = state.read_register::<u8>(s_9_3 as isize);
            tracer.read_register(s_9_3 as isize, value);
            value
        };
        // D s_9_5: cast zx s_9_4 -> bv
        let s_9_5: Bits = Bits::new(s_9_4 as u128, 2u16);
        // D s_9_6: cmp-eq s_9_2 s_9_5
        let s_9_6: bool = ((s_9_2) == (s_9_5));
        // N s_9_7: branch s_9_6 b17 b10
        if s_9_6 {
            return block_17(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var route_to_el2:u8
        let s_10_0: bool = fn_state.route_to_el2;
        // D s_10_1: write-var gs#25245 <= s_10_0
        fn_state.gs_25245 = s_10_0;
        // N s_10_2: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var gs#25245:u8
        let s_11_0: bool = fn_state.gs_25245;
        // N s_11_1: branch s_11_0 b16 b12
        if s_11_0 {
            return block_16(state, tracer, fn_state);
        } else {
            return block_12(state, tracer, fn_state);
        };
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_12_0: const #16975u : u32
        let s_12_0: u32 = 16975;
        // D s_12_1: read-reg s_12_0:u8
        let s_12_1: u8 = {
            let value = state.read_register::<u8>(s_12_0 as isize);
            tracer.read_register(s_12_0 as isize, value);
            value
        };
        // D s_12_2: cast zx s_12_1 -> bv
        let s_12_2: Bits = Bits::new(s_12_1 as u128, 2u16);
        // C s_12_3: const #448u : u32
        let s_12_3: u32 = 448;
        // D s_12_4: read-reg s_12_3:u8
        let s_12_4: u8 = {
            let value = state.read_register::<u8>(s_12_3 as isize);
            tracer.read_register(s_12_3 as isize, value);
            value
        };
        // D s_12_5: cast zx s_12_4 -> bv
        let s_12_5: Bits = Bits::new(s_12_4 as u128, 2u16);
        // D s_12_6: cmp-eq s_12_2 s_12_5
        let s_12_6: bool = ((s_12_2) == (s_12_5));
        // N s_12_7: branch s_12_6 b15 b13
        if s_12_6 {
            return block_15(state, tracer, fn_state);
        } else {
            return block_13(state, tracer, fn_state);
        };
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_13_0: const #16975u : u32
        let s_13_0: u32 = 16975;
        // D s_13_1: read-reg s_13_0:u8
        let s_13_1: u8 = {
            let value = state.read_register::<u8>(s_13_0 as isize);
            tracer.read_register(s_13_0 as isize, value);
            value
        };
        // D s_13_2: cast zx s_13_1 -> bv
        let s_13_2: Bits = Bits::new(s_13_1 as u128, 2u16);
        // C s_13_3: const #440u : u32
        let s_13_3: u32 = 440;
        // D s_13_4: read-reg s_13_3:u8
        let s_13_4: u8 = {
            let value = state.read_register::<u8>(s_13_3 as isize);
            tracer.read_register(s_13_3 as isize, value);
            value
        };
        // D s_13_5: cast zx s_13_4 -> bv
        let s_13_5: Bits = Bits::new(s_13_4 as u128, 2u16);
        // D s_13_6: cmp-eq s_13_2 s_13_5
        let s_13_6: bool = ((s_13_2) == (s_13_5));
        // D s_13_7: write-var gs#25246 <= s_13_6
        fn_state.gs_25246 = s_13_6;
        // N s_13_8: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var gs#25246:u8
        let s_14_0: bool = fn_state.gs_25246;
        // N s_14_1: assert s_14_0
        let s_14_1: () = assert!(s_14_0);
        // D s_14_2: read-var vect_offset:i64
        let s_14_2: i64 = fn_state.vect_offset;
        // D s_14_3: cast zx s_14_2 -> i
        let s_14_3: i128 = (i128::try_from(s_14_2).unwrap());
        // C s_14_4: const #440u : u32
        let s_14_4: u32 = 440;
        // D s_14_5: read-reg s_14_4:u8
        let s_14_5: u8 = {
            let value = state.read_register::<u8>(s_14_4 as isize);
            tracer.read_register(s_14_4 as isize, value);
            value
        };
        // D s_14_6: read-var except:struct
        let s_14_6: ProductTypeb7f99f96751e17c4 = fn_state.except;
        // D s_14_7: read-var preferred_exception_return:u64
        let s_14_7: u64 = fn_state.preferred_exception_return;
        // D s_14_8: call AArch64_TakeException(s_14_5, s_14_6, s_14_7, s_14_3)
        let s_14_8: () = AArch64_TakeException(
            state,
            tracer,
            s_14_5,
            s_14_6,
            s_14_7,
            s_14_3,
        );
        // N s_14_9: return
        return;
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_15_0: const #1u : u8
        let s_15_0: bool = true;
        // D s_15_1: write-var gs#25246 <= s_15_0
        fn_state.gs_25246 = s_15_0;
        // N s_15_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_16_0: const #16975u : u32
        let s_16_0: u32 = 16975;
        // D s_16_1: read-reg s_16_0:u8
        let s_16_1: u8 = {
            let value = state.read_register::<u8>(s_16_0 as isize);
            tracer.read_register(s_16_0 as isize, value);
            value
        };
        // D s_16_2: cast zx s_16_1 -> bv
        let s_16_2: Bits = Bits::new(s_16_1 as u128, 2u16);
        // C s_16_3: const #424u : u32
        let s_16_3: u32 = 424;
        // D s_16_4: read-reg s_16_3:u8
        let s_16_4: u8 = {
            let value = state.read_register::<u8>(s_16_3 as isize);
            tracer.read_register(s_16_3 as isize, value);
            value
        };
        // D s_16_5: cast zx s_16_4 -> bv
        let s_16_5: Bits = Bits::new(s_16_4 as u128, 2u16);
        // D s_16_6: cmp-ne s_16_2 s_16_5
        let s_16_6: bool = ((s_16_2) != (s_16_5));
        // N s_16_7: assert s_16_6
        let s_16_7: () = assert!(s_16_6);
        // D s_16_8: read-var vect_offset:i64
        let s_16_8: i64 = fn_state.vect_offset;
        // D s_16_9: cast zx s_16_8 -> i
        let s_16_9: i128 = (i128::try_from(s_16_8).unwrap());
        // C s_16_10: const #432u : u32
        let s_16_10: u32 = 432;
        // D s_16_11: read-reg s_16_10:u8
        let s_16_11: u8 = {
            let value = state.read_register::<u8>(s_16_10 as isize);
            tracer.read_register(s_16_10 as isize, value);
            value
        };
        // D s_16_12: read-var except:struct
        let s_16_12: ProductTypeb7f99f96751e17c4 = fn_state.except;
        // D s_16_13: read-var preferred_exception_return:u64
        let s_16_13: u64 = fn_state.preferred_exception_return;
        // D s_16_14: call AArch64_TakeException(s_16_11, s_16_12, s_16_13, s_16_9)
        let s_16_14: () = AArch64_TakeException(
            state,
            tracer,
            s_16_11,
            s_16_12,
            s_16_13,
            s_16_9,
        );
        // N s_16_15: return
        return;
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_17_0: const #1u : u8
        let s_17_0: bool = true;
        // D s_17_1: write-var gs#25245 <= s_17_0
        fn_state.gs_25245 = s_17_0;
        // N s_17_2: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_18_0: read-var vect_offset:i64
        let s_18_0: i64 = fn_state.vect_offset;
        // D s_18_1: cast zx s_18_0 -> i
        let s_18_1: i128 = (i128::try_from(s_18_0).unwrap());
        // C s_18_2: const #424u : u32
        let s_18_2: u32 = 424;
        // D s_18_3: read-reg s_18_2:u8
        let s_18_3: u8 = {
            let value = state.read_register::<u8>(s_18_2 as isize);
            tracer.read_register(s_18_2 as isize, value);
            value
        };
        // D s_18_4: read-var except:struct
        let s_18_4: ProductTypeb7f99f96751e17c4 = fn_state.except;
        // D s_18_5: read-var preferred_exception_return:u64
        let s_18_5: u64 = fn_state.preferred_exception_return;
        // D s_18_6: call AArch64_TakeException(s_18_3, s_18_4, s_18_5, s_18_1)
        let s_18_6: () = AArch64_TakeException(
            state,
            tracer,
            s_18_3,
            s_18_4,
            s_18_5,
            s_18_1,
        );
        // N s_18_7: return
        return;
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_19_0: const #102552u : u32
        let s_19_0: u32 = 102552;
        // D s_19_1: read-reg s_19_0:struct
        let s_19_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_19_0 as isize);
            tracer.read_register(s_19_0 as isize, value);
            value
        };
        // D s_19_2: call _get_HCR_EL2_Type_TGE(s_19_1)
        let s_19_2: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_19_1);
        // D s_19_3: cast zx s_19_2 -> bv
        let s_19_3: Bits = Bits::new(s_19_2 as u128, 1u16);
        // C s_19_4: const #1u : u8
        let s_19_4: bool = true;
        // C s_19_5: cast zx s_19_4 -> bv
        let s_19_5: Bits = Bits::new(s_19_4 as u128, 1u16);
        // D s_19_6: cmp-eq s_19_3 s_19_5
        let s_19_6: bool = ((s_19_3) == (s_19_5));
        // N s_19_7: branch s_19_6 b22 b20
        if s_19_6 {
            return block_22(state, tracer, fn_state);
        } else {
            return block_20(state, tracer, fn_state);
        };
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_20_0: const #102552u : u32
        let s_20_0: u32 = 102552;
        // D s_20_1: read-reg s_20_0:struct
        let s_20_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_20_0 as isize);
            tracer.read_register(s_20_0 as isize, value);
            value
        };
        // D s_20_2: call _get_HCR_EL2_Type_IMO(s_20_1)
        let s_20_2: bool = u_get_HCR_EL2_Type_IMO(state, tracer, s_20_1);
        // D s_20_3: cast zx s_20_2 -> bv
        let s_20_3: Bits = Bits::new(s_20_2 as u128, 1u16);
        // C s_20_4: const #1u : u8
        let s_20_4: bool = true;
        // C s_20_5: cast zx s_20_4 -> bv
        let s_20_5: Bits = Bits::new(s_20_4 as u128, 1u16);
        // D s_20_6: cmp-eq s_20_3 s_20_5
        let s_20_6: bool = ((s_20_3) == (s_20_5));
        // D s_20_7: write-var gs#25240 <= s_20_6
        fn_state.gs_25240 = s_20_6;
        // N s_20_8: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_21_0: read-var gs#25240:u8
        let s_21_0: bool = fn_state.gs_25240;
        // D s_21_1: write-var gs#25241 <= s_21_0
        fn_state.gs_25241 = s_21_0;
        // N s_21_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_22_0: const #1u : u8
        let s_22_0: bool = true;
        // D s_22_1: write-var gs#25240 <= s_22_0
        fn_state.gs_25240 = s_22_0;
        // N s_22_2: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_23_0: const #() : ()
        let s_23_0: () = ();
        // S s_23_1: call EL2Enabled(s_23_0)
        let s_23_1: bool = EL2Enabled(state, tracer, s_23_0);
        // D s_23_2: write-var gs#25239 <= s_23_1
        fn_state.gs_25239 = s_23_1;
        // N s_23_3: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_24_0: const #1u : u8
        let s_24_0: bool = true;
        // D s_24_1: write-var gs#25238 <= s_24_0
        fn_state.gs_25238 = s_24_0;
        // N s_24_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_25_0: const #90704u : u32
        let s_25_0: u32 = 90704;
        // D s_25_1: read-reg s_25_0:struct
        let s_25_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_25_0 as isize);
            tracer.read_register(s_25_0 as isize, value);
            value
        };
        // D s_25_2: call _get_SCR_EL3_Type_IRQ(s_25_1)
        let s_25_2: bool = u_get_SCR_EL3_Type_IRQ(state, tracer, s_25_1);
        // D s_25_3: cast zx s_25_2 -> bv
        let s_25_3: Bits = Bits::new(s_25_2 as u128, 1u16);
        // C s_25_4: const #1u : u8
        let s_25_4: bool = true;
        // C s_25_5: cast zx s_25_4 -> bv
        let s_25_5: Bits = Bits::new(s_25_4 as u128, 1u16);
        // D s_25_6: cmp-eq s_25_3 s_25_5
        let s_25_6: bool = ((s_25_3) == (s_25_5));
        // D s_25_7: write-var gs#25237 <= s_25_6
        fn_state.gs_25237 = s_25_6;
        // N s_25_8: jump b2
        return block_2(state, tracer, fn_state);
    }
}
