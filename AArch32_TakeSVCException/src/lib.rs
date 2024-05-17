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
use AArch32_EnterHypMode::*;
use SSAdvance::*;
use HCR_read::*;
use ExceptionSyndrome::*;
use NextInstrAddr::*;
use AArch32_ITAdvance::*;
use EL2Enabled::*;
use u_get_HCR_Type_TGE::*;
use AArch32_EnterMode::*;
use common::*;
pub fn AArch32_TakeSVCException<T: Tracer>(
    state: &mut State,
    tracer: &T,
    immediate: u16,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        route_to_hyp: bool,
        gs_31822: bool,
        except: ProductTypeb7f99f96751e17c4,
        gs_31816: bool,
        preferred_exception_return: u32,
        vect_offset: i64,
        gs_31817: bool,
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
        // C s_0_0: const #() : ()
        let s_0_0: () = ();
        // S s_0_1: call AArch32_ITAdvance(s_0_0)
        let s_0_1: () = AArch32_ITAdvance(state, tracer, s_0_0);
        // C s_0_2: const #() : ()
        let s_0_2: () = ();
        // S s_0_3: call SSAdvance(s_0_2)
        let s_0_3: () = SSAdvance(state, tracer, s_0_2);
        // C s_0_4: const #16975u : u32
        let s_0_4: u32 = 16975;
        // D s_0_5: read-reg s_0_4:u8
        let s_0_5: u8 = {
            let value = state.read_register::<u8>(s_0_4 as isize);
            tracer.read_register(s_0_4 as isize, value);
            value
        };
        // D s_0_6: cast zx s_0_5 -> bv
        let s_0_6: Bits = Bits::new(s_0_5 as u128, 2u16);
        // C s_0_7: const #448u : u32
        let s_0_7: u32 = 448;
        // D s_0_8: read-reg s_0_7:u8
        let s_0_8: u8 = {
            let value = state.read_register::<u8>(s_0_7 as isize);
            tracer.read_register(s_0_7 as isize, value);
            value
        };
        // D s_0_9: cast zx s_0_8 -> bv
        let s_0_9: Bits = Bits::new(s_0_8 as u128, 2u16);
        // D s_0_10: cmp-eq s_0_6 s_0_9
        let s_0_10: bool = ((s_0_6) == (s_0_9));
        // N s_0_11: branch s_0_10 b13 b1
        if s_0_10 {
            return block_13(state, tracer, fn_state);
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
        // D s_1_1: write-var gs#31816 <= s_1_0
        fn_state.gs_31816 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var gs#31816:u8
        let s_2_0: bool = fn_state.gs_31816;
        // N s_2_1: branch s_2_0 b12 b3
        if s_2_0 {
            return block_12(state, tracer, fn_state);
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
        // D s_3_1: write-var gs#31817 <= s_3_0
        fn_state.gs_31817 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var gs#31817:u8
        let s_4_0: bool = fn_state.gs_31817;
        // D s_4_1: write-var route_to_hyp <= s_4_0
        fn_state.route_to_hyp = s_4_0;
        // C s_4_2: const #32s : i64
        let s_4_2: i64 = 32;
        // C s_4_3: cast zx s_4_2 -> i
        let s_4_3: i128 = (i128::try_from(s_4_2).unwrap());
        // S s_4_4: call NextInstrAddr(s_4_3)
        let s_4_4: Bits = NextInstrAddr(state, tracer, s_4_3);
        // S s_4_5: cast reint s_4_4 -> u32
        let s_4_5: u32 = (s_4_4.value() as u32);
        // D s_4_6: write-var preferred_exception_return <= s_4_5
        fn_state.preferred_exception_return = s_4_5;
        // C s_4_7: const #8u : u8
        let s_4_7: u8 = 8;
        // C s_4_8: cast zx s_4_7 -> bv
        let s_4_8: Bits = Bits::new(s_4_7 as u128, 8u16);
        // C s_4_9: cast zx s_4_8 -> i
        let s_4_9: i128 = (s_4_8.value() as i128);
        // C s_4_10: cast reint s_4_9 -> i64
        let s_4_10: i64 = (s_4_9 as i64);
        // D s_4_11: write-var vect_offset <= s_4_10
        fn_state.vect_offset = s_4_10;
        // C s_4_12: const #16975u : u32
        let s_4_12: u32 = 16975;
        // D s_4_13: read-reg s_4_12:u8
        let s_4_13: u8 = {
            let value = state.read_register::<u8>(s_4_12 as isize);
            tracer.read_register(s_4_12 as isize, value);
            value
        };
        // D s_4_14: cast zx s_4_13 -> bv
        let s_4_14: Bits = Bits::new(s_4_13 as u128, 2u16);
        // C s_4_15: const #432u : u32
        let s_4_15: u32 = 432;
        // D s_4_16: read-reg s_4_15:u8
        let s_4_16: u8 = {
            let value = state.read_register::<u8>(s_4_15 as isize);
            tracer.read_register(s_4_15 as isize, value);
            value
        };
        // D s_4_17: cast zx s_4_16 -> bv
        let s_4_17: Bits = Bits::new(s_4_16 as u128, 2u16);
        // D s_4_18: cmp-eq s_4_14 s_4_17
        let s_4_18: bool = ((s_4_14) == (s_4_17));
        // N s_4_19: branch s_4_18 b11 b5
        if s_4_18 {
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
        // D s_5_0: read-var route_to_hyp:u8
        let s_5_0: bool = fn_state.route_to_hyp;
        // D s_5_1: write-var gs#31822 <= s_5_0
        fn_state.gs_31822 = s_5_0;
        // N s_5_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var gs#31822:u8
        let s_6_0: bool = fn_state.gs_31822;
        // N s_6_1: branch s_6_0 b8 b7
        if s_6_0 {
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
        // C s_7_0: const #0s : i64
        let s_7_0: i64 = 0;
        // C s_7_1: cast zx s_7_0 -> i
        let s_7_1: i128 = (i128::try_from(s_7_0).unwrap());
        // D s_7_2: read-var vect_offset:i64
        let s_7_2: i64 = fn_state.vect_offset;
        // D s_7_3: cast zx s_7_2 -> i
        let s_7_3: i128 = (i128::try_from(s_7_2).unwrap());
        // C s_7_4: const #376u : u32
        let s_7_4: u32 = 376;
        // D s_7_5: read-reg s_7_4:u8
        let s_7_5: u8 = {
            let value = state.read_register::<u8>(s_7_4 as isize);
            tracer.read_register(s_7_4 as isize, value);
            value
        };
        // D s_7_6: read-var preferred_exception_return:u32
        let s_7_6: u32 = fn_state.preferred_exception_return;
        // D s_7_7: call AArch32_EnterMode(s_7_5, s_7_6, s_7_1, s_7_3)
        let s_7_7: () = AArch32_EnterMode(state, tracer, s_7_5, s_7_6, s_7_1, s_7_3);
        // N s_7_8: return
        return;
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_8_0: const #12u : u32
        let s_8_0: u32 = 12;
        // S s_8_1: call ExceptionSyndrome(s_8_0)
        let s_8_1: ProductTypeb7f99f96751e17c4 = ExceptionSyndrome(state, tracer, s_8_0);
        // D s_8_2: write-var except <= s_8_1
        fn_state.except = s_8_1;
        // D s_8_3: read-var except:struct
        let s_8_3: ProductTypeb7f99f96751e17c4 = fn_state.except;
        // D s_8_4: write-var except <= s_8_3
        fn_state.except = s_8_3;
        // C s_8_5: const #16975u : u32
        let s_8_5: u32 = 16975;
        // D s_8_6: read-reg s_8_5:u8
        let s_8_6: u8 = {
            let value = state.read_register::<u8>(s_8_5 as isize);
            tracer.read_register(s_8_5 as isize, value);
            value
        };
        // D s_8_7: cast zx s_8_6 -> bv
        let s_8_7: Bits = Bits::new(s_8_6 as u128, 2u16);
        // C s_8_8: const #432u : u32
        let s_8_8: u32 = 432;
        // D s_8_9: read-reg s_8_8:u8
        let s_8_9: u8 = {
            let value = state.read_register::<u8>(s_8_8 as isize);
            tracer.read_register(s_8_8 as isize, value);
            value
        };
        // D s_8_10: cast zx s_8_9 -> bv
        let s_8_10: Bits = Bits::new(s_8_9 as u128, 2u16);
        // D s_8_11: cmp-eq s_8_7 s_8_10
        let s_8_11: bool = ((s_8_7) == (s_8_10));
        // N s_8_12: branch s_8_11 b10 b9
        if s_8_11 {
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
        // C s_9_0: const #20u : u8
        let s_9_0: u8 = 20;
        // C s_9_1: cast zx s_9_0 -> bv
        let s_9_1: Bits = Bits::new(s_9_0 as u128, 8u16);
        // C s_9_2: cast zx s_9_1 -> i
        let s_9_2: i128 = (s_9_1.value() as i128);
        // C s_9_3: cast reint s_9_2 -> i64
        let s_9_3: i64 = (s_9_2 as i64);
        // C s_9_4: cast zx s_9_3 -> i
        let s_9_4: i128 = (i128::try_from(s_9_3).unwrap());
        // D s_9_5: read-var except:struct
        let s_9_5: ProductTypeb7f99f96751e17c4 = fn_state.except;
        // D s_9_6: read-var preferred_exception_return:u32
        let s_9_6: u32 = fn_state.preferred_exception_return;
        // D s_9_7: call AArch32_EnterHypMode(s_9_5, s_9_6, s_9_4)
        let s_9_7: () = AArch32_EnterHypMode(state, tracer, s_9_5, s_9_6, s_9_4);
        // N s_9_8: return
        return;
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var vect_offset:i64
        let s_10_0: i64 = fn_state.vect_offset;
        // D s_10_1: cast zx s_10_0 -> i
        let s_10_1: i128 = (i128::try_from(s_10_0).unwrap());
        // D s_10_2: read-var except:struct
        let s_10_2: ProductTypeb7f99f96751e17c4 = fn_state.except;
        // D s_10_3: read-var preferred_exception_return:u32
        let s_10_3: u32 = fn_state.preferred_exception_return;
        // D s_10_4: call AArch32_EnterHypMode(s_10_2, s_10_3, s_10_1)
        let s_10_4: () = AArch32_EnterHypMode(state, tracer, s_10_2, s_10_3, s_10_1);
        // N s_10_5: return
        return;
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_11_0: const #1u : u8
        let s_11_0: bool = true;
        // D s_11_1: write-var gs#31822 <= s_11_0
        fn_state.gs_31822 = s_11_0;
        // N s_11_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_12_0: const #() : ()
        let s_12_0: () = ();
        // S s_12_1: call HCR_read(s_12_0)
        let s_12_1: ProductType700c18a878c5601b = HCR_read(state, tracer, s_12_0);
        // S s_12_2: call _get_HCR_Type_TGE(s_12_1)
        let s_12_2: bool = u_get_HCR_Type_TGE(state, tracer, s_12_1);
        // S s_12_3: cast zx s_12_2 -> bv
        let s_12_3: Bits = Bits::new(s_12_2 as u128, 1u16);
        // C s_12_4: const #1u : u8
        let s_12_4: bool = true;
        // C s_12_5: cast zx s_12_4 -> bv
        let s_12_5: Bits = Bits::new(s_12_4 as u128, 1u16);
        // S s_12_6: cmp-eq s_12_3 s_12_5
        let s_12_6: bool = ((s_12_3) == (s_12_5));
        // D s_12_7: write-var gs#31817 <= s_12_6
        fn_state.gs_31817 = s_12_6;
        // N s_12_8: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_13_0: const #() : ()
        let s_13_0: () = ();
        // S s_13_1: call EL2Enabled(s_13_0)
        let s_13_1: bool = EL2Enabled(state, tracer, s_13_0);
        // D s_13_2: write-var gs#31816 <= s_13_1
        fn_state.gs_31816 = s_13_1;
        // N s_13_3: jump b2
        return block_2(state, tracer, fn_state);
    }
}
