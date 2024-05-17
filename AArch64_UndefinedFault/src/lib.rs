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
use u_get_HCR_EL2_Type_TGE::*;
use AArch64_TakeException::*;
use ExceptionSyndrome::*;
use EL2Enabled::*;
use common::*;
pub fn AArch64_UndefinedFault<T: Tracer>(
    state: &mut State,
    tracer: &T,
    gs_24652: (),
) -> () {
    #[derive(Default)]
    struct FunctionState {
        route_to_el2: bool,
        gs_24654: bool,
        vect_offset: i64,
        except: ProductTypeb7f99f96751e17c4,
        gs_24653: bool,
        preferred_exception_return: u64,
        gs_24652: (),
    }
    let fn_state = FunctionState {
        gs_24652,
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
        // N s_0_7: branch s_0_6 b10 b1
        if s_0_6 {
            return block_10(state, tracer, fn_state);
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
        // D s_1_1: write-var gs#24653 <= s_1_0
        fn_state.gs_24653 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var gs#24653:u8
        let s_2_0: bool = fn_state.gs_24653;
        // N s_2_1: branch s_2_0 b9 b3
        if s_2_0 {
            return block_9(state, tracer, fn_state);
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
        // D s_3_1: write-var gs#24654 <= s_3_0
        fn_state.gs_24654 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var gs#24654:u8
        let s_4_0: bool = fn_state.gs_24654;
        // D s_4_1: write-var route_to_el2 <= s_4_0
        fn_state.route_to_el2 = s_4_0;
        // C s_4_2: const #64s : i64
        let s_4_2: i64 = 64;
        // C s_4_3: cast zx s_4_2 -> i
        let s_4_3: i128 = (i128::try_from(s_4_2).unwrap());
        // S s_4_4: call ThisInstrAddr(s_4_3)
        let s_4_4: Bits = ThisInstrAddr(state, tracer, s_4_3);
        // S s_4_5: cast reint s_4_4 -> u64
        let s_4_5: u64 = (s_4_4.value() as u64);
        // D s_4_6: write-var preferred_exception_return <= s_4_5
        fn_state.preferred_exception_return = s_4_5;
        // C s_4_7: const #0u : u8
        let s_4_7: u8 = 0;
        // C s_4_8: cast zx s_4_7 -> bv
        let s_4_8: Bits = Bits::new(s_4_7 as u128, 4u16);
        // C s_4_9: cast zx s_4_8 -> i
        let s_4_9: i128 = (s_4_8.value() as i128);
        // C s_4_10: cast reint s_4_9 -> i64
        let s_4_10: i64 = (s_4_9 as i64);
        // D s_4_11: write-var vect_offset <= s_4_10
        fn_state.vect_offset = s_4_10;
        // C s_4_12: const #0u : u32
        let s_4_12: u32 = 0;
        // S s_4_13: call ExceptionSyndrome(s_4_12)
        let s_4_13: ProductTypeb7f99f96751e17c4 = ExceptionSyndrome(
            state,
            tracer,
            s_4_12,
        );
        // D s_4_14: write-var except <= s_4_13
        fn_state.except = s_4_13;
        // C s_4_15: const #16975u : u32
        let s_4_15: u32 = 16975;
        // D s_4_16: read-reg s_4_15:u8
        let s_4_16: u8 = {
            let value = state.read_register::<u8>(s_4_15 as isize);
            tracer.read_register(s_4_15 as isize, value);
            value
        };
        // D s_4_17: cast zx s_4_16 -> bv
        let s_4_17: Bits = Bits::new(s_4_16 as u128, 2u16);
        // D s_4_18: cast zx s_4_17 -> i
        let s_4_18: i128 = (s_4_17.value() as i128);
        // D s_4_19: cast reint s_4_18 -> i64
        let s_4_19: i64 = (s_4_18 as i64);
        // C s_4_20: const #440u : u32
        let s_4_20: u32 = 440;
        // D s_4_21: read-reg s_4_20:u8
        let s_4_21: u8 = {
            let value = state.read_register::<u8>(s_4_20 as isize);
            tracer.read_register(s_4_20 as isize, value);
            value
        };
        // D s_4_22: cast zx s_4_21 -> bv
        let s_4_22: Bits = Bits::new(s_4_21 as u128, 2u16);
        // D s_4_23: cast zx s_4_22 -> i
        let s_4_23: i128 = (s_4_22.value() as i128);
        // D s_4_24: cast reint s_4_23 -> i64
        let s_4_24: i64 = (s_4_23 as i64);
        // D s_4_25: cast zx s_4_19 -> i
        let s_4_25: i128 = (i128::try_from(s_4_19).unwrap());
        // D s_4_26: cast zx s_4_24 -> i
        let s_4_26: i128 = (i128::try_from(s_4_24).unwrap());
        // D s_4_27: cmp-gt s_4_25 s_4_26
        let s_4_27: bool = ((s_4_25) > (s_4_26));
        // N s_4_28: branch s_4_27 b8 b5
        if s_4_27 {
            return block_8(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var route_to_el2:u8
        let s_5_0: bool = fn_state.route_to_el2;
        // N s_5_1: branch s_5_0 b7 b6
        if s_5_0 {
            return block_7(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var vect_offset:i64
        let s_6_0: i64 = fn_state.vect_offset;
        // D s_6_1: cast zx s_6_0 -> i
        let s_6_1: i128 = (i128::try_from(s_6_0).unwrap());
        // C s_6_2: const #440u : u32
        let s_6_2: u32 = 440;
        // D s_6_3: read-reg s_6_2:u8
        let s_6_3: u8 = {
            let value = state.read_register::<u8>(s_6_2 as isize);
            tracer.read_register(s_6_2 as isize, value);
            value
        };
        // D s_6_4: read-var except:struct
        let s_6_4: ProductTypeb7f99f96751e17c4 = fn_state.except;
        // D s_6_5: read-var preferred_exception_return:u64
        let s_6_5: u64 = fn_state.preferred_exception_return;
        // D s_6_6: call AArch64_TakeException(s_6_3, s_6_4, s_6_5, s_6_1)
        let s_6_6: () = AArch64_TakeException(state, tracer, s_6_3, s_6_4, s_6_5, s_6_1);
        // N s_6_7: return
        return;
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var vect_offset:i64
        let s_7_0: i64 = fn_state.vect_offset;
        // D s_7_1: cast zx s_7_0 -> i
        let s_7_1: i128 = (i128::try_from(s_7_0).unwrap());
        // C s_7_2: const #432u : u32
        let s_7_2: u32 = 432;
        // D s_7_3: read-reg s_7_2:u8
        let s_7_3: u8 = {
            let value = state.read_register::<u8>(s_7_2 as isize);
            tracer.read_register(s_7_2 as isize, value);
            value
        };
        // D s_7_4: read-var except:struct
        let s_7_4: ProductTypeb7f99f96751e17c4 = fn_state.except;
        // D s_7_5: read-var preferred_exception_return:u64
        let s_7_5: u64 = fn_state.preferred_exception_return;
        // D s_7_6: call AArch64_TakeException(s_7_3, s_7_4, s_7_5, s_7_1)
        let s_7_6: () = AArch64_TakeException(state, tracer, s_7_3, s_7_4, s_7_5, s_7_1);
        // N s_7_7: return
        return;
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_8_0: const #16975u : u32
        let s_8_0: u32 = 16975;
        // D s_8_1: read-reg s_8_0:u8
        let s_8_1: u8 = {
            let value = state.read_register::<u8>(s_8_0 as isize);
            tracer.read_register(s_8_0 as isize, value);
            value
        };
        // D s_8_2: read-var vect_offset:i64
        let s_8_2: i64 = fn_state.vect_offset;
        // D s_8_3: cast zx s_8_2 -> i
        let s_8_3: i128 = (i128::try_from(s_8_2).unwrap());
        // D s_8_4: read-var except:struct
        let s_8_4: ProductTypeb7f99f96751e17c4 = fn_state.except;
        // D s_8_5: read-var preferred_exception_return:u64
        let s_8_5: u64 = fn_state.preferred_exception_return;
        // D s_8_6: call AArch64_TakeException(s_8_1, s_8_4, s_8_5, s_8_3)
        let s_8_6: () = AArch64_TakeException(state, tracer, s_8_1, s_8_4, s_8_5, s_8_3);
        // N s_8_7: return
        return;
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_9_0: const #102552u : u32
        let s_9_0: u32 = 102552;
        // D s_9_1: read-reg s_9_0:struct
        let s_9_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_9_0 as isize);
            tracer.read_register(s_9_0 as isize, value);
            value
        };
        // D s_9_2: call _get_HCR_EL2_Type_TGE(s_9_1)
        let s_9_2: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_9_1);
        // D s_9_3: cast zx s_9_2 -> bv
        let s_9_3: Bits = Bits::new(s_9_2 as u128, 1u16);
        // C s_9_4: const #1u : u8
        let s_9_4: bool = true;
        // C s_9_5: cast zx s_9_4 -> bv
        let s_9_5: Bits = Bits::new(s_9_4 as u128, 1u16);
        // D s_9_6: cmp-eq s_9_3 s_9_5
        let s_9_6: bool = ((s_9_3) == (s_9_5));
        // D s_9_7: write-var gs#24654 <= s_9_6
        fn_state.gs_24654 = s_9_6;
        // N s_9_8: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_10_0: const #() : ()
        let s_10_0: () = ();
        // S s_10_1: call EL2Enabled(s_10_0)
        let s_10_1: bool = EL2Enabled(state, tracer, s_10_0);
        // D s_10_2: write-var gs#24653 <= s_10_1
        fn_state.gs_24653 = s_10_1;
        // N s_10_3: jump b2
        return block_2(state, tracer, fn_state);
    }
}
