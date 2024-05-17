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
use Zeros::*;
use u_get_HCR_EL2_Type_TGE::*;
use AArch64_TakeException::*;
use ExceptionSyndrome::*;
use EL2Enabled::*;
use common::*;
pub fn AArch64_BranchTargetException<T: Tracer>(
    state: &mut State,
    tracer: &T,
    vaddress: u64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        except: ProductTypeb7f99f96751e17c4,
        preferred_exception_return: u64,
        gs_6501: bool,
        vect_offset: i64,
        gs_6502: bool,
        target_el: u8,
        vaddress: u64,
    }
    let fn_state = FunctionState {
        vaddress,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_0_0: const #64s : i64
        let s_0_0: i64 = 64;
        // C s_0_1: cast zx s_0_0 -> i
        let s_0_1: i128 = (i128::try_from(s_0_0).unwrap());
        // S s_0_2: call ThisInstrAddr(s_0_1)
        let s_0_2: Bits = ThisInstrAddr(state, tracer, s_0_1);
        // S s_0_3: cast reint s_0_2 -> u64
        let s_0_3: u64 = (s_0_2.value() as u64);
        // D s_0_4: write-var preferred_exception_return <= s_0_3
        fn_state.preferred_exception_return = s_0_3;
        // C s_0_5: const #0u : u8
        let s_0_5: u8 = 0;
        // C s_0_6: cast zx s_0_5 -> bv
        let s_0_6: Bits = Bits::new(s_0_5 as u128, 4u16);
        // C s_0_7: cast zx s_0_6 -> i
        let s_0_7: i128 = (s_0_6.value() as i128);
        // C s_0_8: cast reint s_0_7 -> i64
        let s_0_8: i64 = (s_0_7 as i64);
        // D s_0_9: write-var vect_offset <= s_0_8
        fn_state.vect_offset = s_0_8;
        // C s_0_10: const #36u : u32
        let s_0_10: u32 = 36;
        // S s_0_11: call ExceptionSyndrome(s_0_10)
        let s_0_11: ProductTypeb7f99f96751e17c4 = ExceptionSyndrome(
            state,
            tracer,
            s_0_10,
        );
        // D s_0_12: write-var except <= s_0_11
        fn_state.except = s_0_11;
        // D s_0_13: read-var except:struct
        let s_0_13: ProductTypeb7f99f96751e17c4 = fn_state.except;
        // D s_0_14: write-var except <= s_0_13
        fn_state.except = s_0_13;
        // C s_0_15: const #23s : i
        let s_0_15: i128 = 23;
        // S s_0_16: call Zeros(s_0_15)
        let s_0_16: Bits = Zeros(state, tracer, s_0_15);
        // D s_0_17: read-var except:struct
        let s_0_17: ProductTypeb7f99f96751e17c4 = fn_state.except;
        // D s_0_18: write-var except <= s_0_17
        fn_state.except = s_0_17;
        // C s_0_19: const #440u : u32
        let s_0_19: u32 = 440;
        // D s_0_20: read-reg s_0_19:u8
        let s_0_20: u8 = {
            let value = state.read_register::<u8>(s_0_19 as isize);
            tracer.read_register(s_0_19 as isize, value);
            value
        };
        // D s_0_21: write-var target_el <= s_0_20
        fn_state.target_el = s_0_20;
        // C s_0_22: const #16975u : u32
        let s_0_22: u32 = 16975;
        // D s_0_23: read-reg s_0_22:u8
        let s_0_23: u8 = {
            let value = state.read_register::<u8>(s_0_22 as isize);
            tracer.read_register(s_0_22 as isize, value);
            value
        };
        // D s_0_24: cast zx s_0_23 -> bv
        let s_0_24: Bits = Bits::new(s_0_23 as u128, 2u16);
        // D s_0_25: cast zx s_0_24 -> i
        let s_0_25: i128 = (s_0_24.value() as i128);
        // D s_0_26: cast reint s_0_25 -> i64
        let s_0_26: i64 = (s_0_25 as i64);
        // C s_0_27: const #440u : u32
        let s_0_27: u32 = 440;
        // D s_0_28: read-reg s_0_27:u8
        let s_0_28: u8 = {
            let value = state.read_register::<u8>(s_0_27 as isize);
            tracer.read_register(s_0_27 as isize, value);
            value
        };
        // D s_0_29: cast zx s_0_28 -> bv
        let s_0_29: Bits = Bits::new(s_0_28 as u128, 2u16);
        // D s_0_30: cast zx s_0_29 -> i
        let s_0_30: i128 = (s_0_29.value() as i128);
        // D s_0_31: cast reint s_0_30 -> i64
        let s_0_31: i64 = (s_0_30 as i64);
        // D s_0_32: cast zx s_0_26 -> i
        let s_0_32: i128 = (i128::try_from(s_0_26).unwrap());
        // D s_0_33: cast zx s_0_31 -> i
        let s_0_33: i128 = (i128::try_from(s_0_31).unwrap());
        // D s_0_34: cmp-gt s_0_32 s_0_33
        let s_0_34: bool = ((s_0_32) > (s_0_33));
        // N s_0_35: branch s_0_34 b11 b1
        if s_0_34 {
            return block_11(state, tracer, fn_state);
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
        // C s_1_3: const #448u : u32
        let s_1_3: u32 = 448;
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
        // N s_1_7: branch s_1_6 b10 b2
        if s_1_6 {
            return block_10(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_2_0: const #0u : u8
        let s_2_0: bool = false;
        // D s_2_1: write-var gs#6501 <= s_2_0
        fn_state.gs_6501 = s_2_0;
        // N s_2_2: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var gs#6501:u8
        let s_3_0: bool = fn_state.gs_6501;
        // N s_3_1: branch s_3_0 b9 b4
        if s_3_0 {
            return block_9(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_4_0: const #0u : u8
        let s_4_0: bool = false;
        // D s_4_1: write-var gs#6502 <= s_4_0
        fn_state.gs_6502 = s_4_0;
        // N s_4_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var gs#6502:u8
        let s_5_0: bool = fn_state.gs_6502;
        // N s_5_1: branch s_5_0 b8 b6
        if s_5_0 {
            return block_8(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_6_0: jump b7
        return block_7(state, tracer, fn_state);
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
        // D s_7_2: read-var target_el:u8
        let s_7_2: u8 = fn_state.target_el;
        // D s_7_3: read-var except:struct
        let s_7_3: ProductTypeb7f99f96751e17c4 = fn_state.except;
        // D s_7_4: read-var preferred_exception_return:u64
        let s_7_4: u64 = fn_state.preferred_exception_return;
        // D s_7_5: call AArch64_TakeException(s_7_2, s_7_3, s_7_4, s_7_1)
        let s_7_5: () = AArch64_TakeException(state, tracer, s_7_2, s_7_3, s_7_4, s_7_1);
        // N s_7_6: return
        return;
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_8_0: const #432u : u32
        let s_8_0: u32 = 432;
        // D s_8_1: read-reg s_8_0:u8
        let s_8_1: u8 = {
            let value = state.read_register::<u8>(s_8_0 as isize);
            tracer.read_register(s_8_0 as isize, value);
            value
        };
        // D s_8_2: write-var target_el <= s_8_1
        fn_state.target_el = s_8_1;
        // N s_8_3: jump b7
        return block_7(state, tracer, fn_state);
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
        // D s_9_7: write-var gs#6502 <= s_9_6
        fn_state.gs_6502 = s_9_6;
        // N s_9_8: jump b5
        return block_5(state, tracer, fn_state);
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
        // D s_10_2: write-var gs#6501 <= s_10_1
        fn_state.gs_6501 = s_10_1;
        // N s_10_3: jump b3
        return block_3(state, tracer, fn_state);
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
        // D s_11_2: write-var target_el <= s_11_1
        fn_state.target_el = s_11_1;
        // N s_11_3: jump b7
        return block_7(state, tracer, fn_state);
    }
}
