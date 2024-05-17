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
use u_get_MDCR_EL2_Type_TDE::*;
use u_get_HCR_EL2_Type_TGE::*;
use AArch64_TakeException::*;
use ExceptionSyndrome::*;
use EL2Enabled::*;
use common::*;
pub fn AArch64_SoftwareBreakpoint<T: Tracer>(
    state: &mut State,
    tracer: &T,
    immediate: u16,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        route_to_el2: bool,
        gs_25282: bool,
        except: ProductTypeb7f99f96751e17c4,
        preferred_exception_return: u64,
        gs_25280: bool,
        vect_offset: i64,
        gs_25281: bool,
        gs_25283: bool,
        immediate: u16,
    }
    let fn_state = FunctionState {
        immediate,
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
        // N s_0_7: branch s_0_6 b16 b1
        if s_0_6 {
            return block_16(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_1_0: const #16975u : u32
        let s_1_0: u32 = 16975;
        // D s_1_1: read-reg s_1_0:u8
        let s_1_1: u8 = {
            let value = state.read_register::<u8>(s_1_0 as isize);
            tracer.read_register(s_1_0 as isize, value);
            value
        };
        // D s_1_2: cast zx s_1_1 -> bv
        let s_1_2: Bits = Bits::new(s_1_1 as u128, 2u16);
        // C s_1_3: const #440u : u32
        let s_1_3: u32 = 440;
        // D s_1_4: read-reg s_1_3:u8
        let s_1_4: u8 = {
            let value = state.read_register::<u8>(s_1_3 as isize);
            tracer.read_register(s_1_3 as isize, value);
            value
        };
        // D s_1_5: cast zx s_1_4 -> bv
        let s_1_5: Bits = Bits::new(s_1_4 as u128, 2u16);
        // D s_1_6: cmp-eq s_1_2 s_1_5
        let s_1_6: bool = ((s_1_2) == (s_1_5));
        // D s_1_7: write-var gs#25280 <= s_1_6
        fn_state.gs_25280 = s_1_6;
        // N s_1_8: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var gs#25280:u8
        let s_2_0: bool = fn_state.gs_25280;
        // N s_2_1: branch s_2_0 b15 b3
        if s_2_0 {
            return block_15(state, tracer, fn_state);
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
        // D s_3_1: write-var gs#25281 <= s_3_0
        fn_state.gs_25281 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var gs#25281:u8
        let s_4_0: bool = fn_state.gs_25281;
        // N s_4_1: branch s_4_0 b11 b5
        if s_4_0 {
            return block_11(state, tracer, fn_state);
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
        // D s_5_1: write-var gs#25283 <= s_5_0
        fn_state.gs_25283 = s_5_0;
        // N s_5_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var gs#25283:u8
        let s_6_0: bool = fn_state.gs_25283;
        // D s_6_1: write-var route_to_el2 <= s_6_0
        fn_state.route_to_el2 = s_6_0;
        // C s_6_2: const #64s : i64
        let s_6_2: i64 = 64;
        // C s_6_3: cast zx s_6_2 -> i
        let s_6_3: i128 = (i128::try_from(s_6_2).unwrap());
        // S s_6_4: call ThisInstrAddr(s_6_3)
        let s_6_4: Bits = ThisInstrAddr(state, tracer, s_6_3);
        // S s_6_5: cast reint s_6_4 -> u64
        let s_6_5: u64 = (s_6_4.value() as u64);
        // D s_6_6: write-var preferred_exception_return <= s_6_5
        fn_state.preferred_exception_return = s_6_5;
        // C s_6_7: const #0u : u8
        let s_6_7: u8 = 0;
        // C s_6_8: cast zx s_6_7 -> bv
        let s_6_8: Bits = Bits::new(s_6_7 as u128, 4u16);
        // C s_6_9: cast zx s_6_8 -> i
        let s_6_9: i128 = (s_6_8.value() as i128);
        // C s_6_10: cast reint s_6_9 -> i64
        let s_6_10: i64 = (s_6_9 as i64);
        // D s_6_11: write-var vect_offset <= s_6_10
        fn_state.vect_offset = s_6_10;
        // C s_6_12: const #29u : u32
        let s_6_12: u32 = 29;
        // S s_6_13: call ExceptionSyndrome(s_6_12)
        let s_6_13: ProductTypeb7f99f96751e17c4 = ExceptionSyndrome(
            state,
            tracer,
            s_6_12,
        );
        // D s_6_14: write-var except <= s_6_13
        fn_state.except = s_6_13;
        // D s_6_15: read-var except:struct
        let s_6_15: ProductTypeb7f99f96751e17c4 = fn_state.except;
        // D s_6_16: write-var except <= s_6_15
        fn_state.except = s_6_15;
        // C s_6_17: const #16975u : u32
        let s_6_17: u32 = 16975;
        // D s_6_18: read-reg s_6_17:u8
        let s_6_18: u8 = {
            let value = state.read_register::<u8>(s_6_17 as isize);
            tracer.read_register(s_6_17 as isize, value);
            value
        };
        // D s_6_19: cast zx s_6_18 -> bv
        let s_6_19: Bits = Bits::new(s_6_18 as u128, 2u16);
        // D s_6_20: cast zx s_6_19 -> i
        let s_6_20: i128 = (s_6_19.value() as i128);
        // D s_6_21: cast reint s_6_20 -> i64
        let s_6_21: i64 = (s_6_20 as i64);
        // C s_6_22: const #440u : u32
        let s_6_22: u32 = 440;
        // D s_6_23: read-reg s_6_22:u8
        let s_6_23: u8 = {
            let value = state.read_register::<u8>(s_6_22 as isize);
            tracer.read_register(s_6_22 as isize, value);
            value
        };
        // D s_6_24: cast zx s_6_23 -> bv
        let s_6_24: Bits = Bits::new(s_6_23 as u128, 2u16);
        // D s_6_25: cast zx s_6_24 -> i
        let s_6_25: i128 = (s_6_24.value() as i128);
        // D s_6_26: cast reint s_6_25 -> i64
        let s_6_26: i64 = (s_6_25 as i64);
        // D s_6_27: cast zx s_6_21 -> i
        let s_6_27: i128 = (i128::try_from(s_6_21).unwrap());
        // D s_6_28: cast zx s_6_26 -> i
        let s_6_28: i128 = (i128::try_from(s_6_26).unwrap());
        // D s_6_29: cmp-gt s_6_27 s_6_28
        let s_6_29: bool = ((s_6_27) > (s_6_28));
        // N s_6_30: branch s_6_29 b10 b7
        if s_6_29 {
            return block_10(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var route_to_el2:u8
        let s_7_0: bool = fn_state.route_to_el2;
        // N s_7_1: branch s_7_0 b9 b8
        if s_7_0 {
            return block_9(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var vect_offset:i64
        let s_8_0: i64 = fn_state.vect_offset;
        // D s_8_1: cast zx s_8_0 -> i
        let s_8_1: i128 = (i128::try_from(s_8_0).unwrap());
        // C s_8_2: const #440u : u32
        let s_8_2: u32 = 440;
        // D s_8_3: read-reg s_8_2:u8
        let s_8_3: u8 = {
            let value = state.read_register::<u8>(s_8_2 as isize);
            tracer.read_register(s_8_2 as isize, value);
            value
        };
        // D s_8_4: read-var except:struct
        let s_8_4: ProductTypeb7f99f96751e17c4 = fn_state.except;
        // D s_8_5: read-var preferred_exception_return:u64
        let s_8_5: u64 = fn_state.preferred_exception_return;
        // D s_8_6: call AArch64_TakeException(s_8_3, s_8_4, s_8_5, s_8_1)
        let s_8_6: () = AArch64_TakeException(state, tracer, s_8_3, s_8_4, s_8_5, s_8_1);
        // N s_8_7: return
        return;
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var vect_offset:i64
        let s_9_0: i64 = fn_state.vect_offset;
        // D s_9_1: cast zx s_9_0 -> i
        let s_9_1: i128 = (i128::try_from(s_9_0).unwrap());
        // C s_9_2: const #432u : u32
        let s_9_2: u32 = 432;
        // D s_9_3: read-reg s_9_2:u8
        let s_9_3: u8 = {
            let value = state.read_register::<u8>(s_9_2 as isize);
            tracer.read_register(s_9_2 as isize, value);
            value
        };
        // D s_9_4: read-var except:struct
        let s_9_4: ProductTypeb7f99f96751e17c4 = fn_state.except;
        // D s_9_5: read-var preferred_exception_return:u64
        let s_9_5: u64 = fn_state.preferred_exception_return;
        // D s_9_6: call AArch64_TakeException(s_9_3, s_9_4, s_9_5, s_9_1)
        let s_9_6: () = AArch64_TakeException(state, tracer, s_9_3, s_9_4, s_9_5, s_9_1);
        // N s_9_7: return
        return;
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_10_0: const #16975u : u32
        let s_10_0: u32 = 16975;
        // D s_10_1: read-reg s_10_0:u8
        let s_10_1: u8 = {
            let value = state.read_register::<u8>(s_10_0 as isize);
            tracer.read_register(s_10_0 as isize, value);
            value
        };
        // D s_10_2: read-var vect_offset:i64
        let s_10_2: i64 = fn_state.vect_offset;
        // D s_10_3: cast zx s_10_2 -> i
        let s_10_3: i128 = (i128::try_from(s_10_2).unwrap());
        // D s_10_4: read-var except:struct
        let s_10_4: ProductTypeb7f99f96751e17c4 = fn_state.except;
        // D s_10_5: read-var preferred_exception_return:u64
        let s_10_5: u64 = fn_state.preferred_exception_return;
        // D s_10_6: call AArch64_TakeException(s_10_1, s_10_4, s_10_5, s_10_3)
        let s_10_6: () = AArch64_TakeException(
            state,
            tracer,
            s_10_1,
            s_10_4,
            s_10_5,
            s_10_3,
        );
        // N s_10_7: return
        return;
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_11_0: const #102552u : u32
        let s_11_0: u32 = 102552;
        // D s_11_1: read-reg s_11_0:struct
        let s_11_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_11_0 as isize);
            tracer.read_register(s_11_0 as isize, value);
            value
        };
        // D s_11_2: call _get_HCR_EL2_Type_TGE(s_11_1)
        let s_11_2: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_11_1);
        // D s_11_3: cast zx s_11_2 -> bv
        let s_11_3: Bits = Bits::new(s_11_2 as u128, 1u16);
        // C s_11_4: const #1u : u8
        let s_11_4: bool = true;
        // C s_11_5: cast zx s_11_4 -> bv
        let s_11_5: Bits = Bits::new(s_11_4 as u128, 1u16);
        // D s_11_6: cmp-eq s_11_3 s_11_5
        let s_11_6: bool = ((s_11_3) == (s_11_5));
        // N s_11_7: branch s_11_6 b14 b12
        if s_11_6 {
            return block_14(state, tracer, fn_state);
        } else {
            return block_12(state, tracer, fn_state);
        };
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_12_0: const #104880u : u32
        let s_12_0: u32 = 104880;
        // D s_12_1: read-reg s_12_0:struct
        let s_12_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_12_0 as isize);
            tracer.read_register(s_12_0 as isize, value);
            value
        };
        // D s_12_2: call _get_MDCR_EL2_Type_TDE(s_12_1)
        let s_12_2: bool = u_get_MDCR_EL2_Type_TDE(state, tracer, s_12_1);
        // D s_12_3: cast zx s_12_2 -> bv
        let s_12_3: Bits = Bits::new(s_12_2 as u128, 1u16);
        // C s_12_4: const #1u : u8
        let s_12_4: bool = true;
        // C s_12_5: cast zx s_12_4 -> bv
        let s_12_5: Bits = Bits::new(s_12_4 as u128, 1u16);
        // D s_12_6: cmp-eq s_12_3 s_12_5
        let s_12_6: bool = ((s_12_3) == (s_12_5));
        // D s_12_7: write-var gs#25282 <= s_12_6
        fn_state.gs_25282 = s_12_6;
        // N s_12_8: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var gs#25282:u8
        let s_13_0: bool = fn_state.gs_25282;
        // D s_13_1: write-var gs#25283 <= s_13_0
        fn_state.gs_25283 = s_13_0;
        // N s_13_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_14_0: const #1u : u8
        let s_14_0: bool = true;
        // D s_14_1: write-var gs#25282 <= s_14_0
        fn_state.gs_25282 = s_14_0;
        // N s_14_2: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_15_0: const #() : ()
        let s_15_0: () = ();
        // S s_15_1: call EL2Enabled(s_15_0)
        let s_15_1: bool = EL2Enabled(state, tracer, s_15_0);
        // D s_15_2: write-var gs#25281 <= s_15_1
        fn_state.gs_25281 = s_15_1;
        // N s_15_3: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_16_0: const #1u : u8
        let s_16_0: bool = true;
        // D s_16_1: write-var gs#25280 <= s_16_0
        fn_state.gs_25280 = s_16_0;
        // N s_16_2: jump b2
        return block_2(state, tracer, fn_state);
    }
}
