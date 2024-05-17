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
use u__UNKNOWN_bits::*;
use AArch64_TakeException::*;
use ExceptionSyndrome::*;
use Bit::*;
use u__IMPDEF_boolean::*;
use EL2Enabled::*;
use u_get_HCR_EL2_Type_TGE::*;
use common::*;
pub fn AArch64_FPTrappedException<T: Tracer>(
    state: &mut State,
    tracer: &T,
    is_ase: bool,
    accumulated_exceptions: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        route_to_el2: bool,
        except: ProductTypeb7f99f96751e17c4,
        preferred_exception_return: u64,
        vect_offset: i64,
        gs_10152: bool,
        is_ase: bool,
        accumulated_exceptions: u8,
    }
    let fn_state = FunctionState {
        is_ase,
        accumulated_exceptions,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_0_0: const #23u : u32
        let s_0_0: u32 = 23;
        // S s_0_1: call ExceptionSyndrome(s_0_0)
        let s_0_1: ProductTypeb7f99f96751e17c4 = ExceptionSyndrome(state, tracer, s_0_0);
        // D s_0_2: write-var except <= s_0_1
        fn_state.except = s_0_1;
        // D s_0_3: read-var is_ase:u8
        let s_0_3: bool = fn_state.is_ase;
        // N s_0_4: branch s_0_3 b13 b1
        if s_0_3 {
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
        // C s_1_0: const #1u : u8
        let s_1_0: bool = true;
        // S s_1_1: call Bit(s_1_0)
        let s_1_1: bool = Bit(state, tracer, s_1_0);
        // D s_1_2: read-var except:struct
        let s_1_2: ProductTypeb7f99f96751e17c4 = fn_state.except;
        // D s_1_3: write-var except <= s_1_2
        fn_state.except = s_1_2;
        // N s_1_4: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_2_0: const #3s : i64
        let s_2_0: i64 = 3;
        // C s_2_1: cast zx s_2_0 -> i
        let s_2_1: i128 = (i128::try_from(s_2_0).unwrap());
        // S s_2_2: call __UNKNOWN_bits(s_2_1)
        let s_2_2: Bits = u__UNKNOWN_bits(state, tracer, s_2_1);
        // D s_2_3: read-var except:struct
        let s_2_3: ProductTypeb7f99f96751e17c4 = fn_state.except;
        // D s_2_4: write-var except <= s_2_3
        fn_state.except = s_2_3;
        // D s_2_5: read-var except.6:struct
        let s_2_5: u32 = fn_state.except._6;
        // C s_2_6: const #23s : i
        let s_2_6: i128 = 23;
        // D s_2_7: cast zx s_2_5 -> bv
        let s_2_7: Bits = Bits::new(s_2_5 as u128, 25u16);
        // C s_2_8: const #1u : u64
        let s_2_8: u64 = 1;
        // D s_2_9: bit-extract s_2_7 s_2_6 s_2_8
        let s_2_9: Bits = (Bits::new(
            ((s_2_7) >> (s_2_6)).value(),
            u16::try_from(s_2_8).unwrap(),
        ));
        // D s_2_10: cast reint s_2_9 -> u8
        let s_2_10: bool = ((s_2_9.value()) != 0);
        // C s_2_11: const #0s : i
        let s_2_11: i128 = 0;
        // C s_2_12: const #0u : u64
        let s_2_12: u64 = 0;
        // D s_2_13: cast zx s_2_10 -> u64
        let s_2_13: u64 = (s_2_10 as u64);
        // C s_2_14: const #1u : u64
        let s_2_14: u64 = 1;
        // D s_2_15: and s_2_13 s_2_14
        let s_2_15: u64 = ((s_2_13) & (s_2_14));
        // D s_2_16: cmp-eq s_2_15 s_2_14
        let s_2_16: bool = ((s_2_15) == (s_2_14));
        // D s_2_17: lsl s_2_13 s_2_11
        let s_2_17: u64 = s_2_13 << s_2_11;
        // D s_2_18: or s_2_12 s_2_17
        let s_2_18: u64 = ((s_2_12) | (s_2_17));
        // D s_2_19: cmpl s_2_17
        let s_2_19: u64 = !s_2_17;
        // D s_2_20: and s_2_12 s_2_19
        let s_2_20: u64 = ((s_2_12) & (s_2_19));
        // D s_2_21: select s_2_16 s_2_18 s_2_20
        let s_2_21: u64 = if s_2_16 { s_2_18 } else { s_2_20 };
        // D s_2_22: cast trunc s_2_21 -> u8
        let s_2_22: bool = ((s_2_21) != 0);
        // D s_2_23: cast zx s_2_22 -> bv
        let s_2_23: Bits = Bits::new(s_2_22 as u128, 1u16);
        // C s_2_24: const #1u : u8
        let s_2_24: bool = true;
        // C s_2_25: cast zx s_2_24 -> bv
        let s_2_25: Bits = Bits::new(s_2_24 as u128, 1u16);
        // D s_2_26: cmp-eq s_2_23 s_2_25
        let s_2_26: bool = ((s_2_23) == (s_2_25));
        // N s_2_27: branch s_2_26 b12 b3
        if s_2_26 {
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
        // C s_3_0: const #6s : i64
        let s_3_0: i64 = 6;
        // C s_3_1: cast zx s_3_0 -> i
        let s_3_1: i128 = (i128::try_from(s_3_0).unwrap());
        // S s_3_2: call __UNKNOWN_bits(s_3_1)
        let s_3_2: Bits = u__UNKNOWN_bits(state, tracer, s_3_1);
        // D s_3_3: read-var except:struct
        let s_3_3: ProductTypeb7f99f96751e17c4 = fn_state.except;
        // D s_3_4: write-var except <= s_3_3
        fn_state.except = s_3_3;
        // D s_3_5: read-var except:struct
        let s_3_5: ProductTypeb7f99f96751e17c4 = fn_state.except;
        // D s_3_6: write-var except <= s_3_5
        fn_state.except = s_3_5;
        // N s_3_7: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_4_0: const #() : ()
        let s_4_0: () = ();
        // S s_4_1: call EL2Enabled(s_4_0)
        let s_4_1: bool = EL2Enabled(state, tracer, s_4_0);
        // N s_4_2: branch s_4_1 b11 b5
        if s_4_1 {
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
        // D s_5_1: write-var gs#10152 <= s_5_0
        fn_state.gs_10152 = s_5_0;
        // N s_5_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var gs#10152:u8
        let s_6_0: bool = fn_state.gs_10152;
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
        // C s_6_12: const #16975u : u32
        let s_6_12: u32 = 16975;
        // D s_6_13: read-reg s_6_12:u8
        let s_6_13: u8 = {
            let value = state.read_register::<u8>(s_6_12 as isize);
            tracer.read_register(s_6_12 as isize, value);
            value
        };
        // D s_6_14: cast zx s_6_13 -> bv
        let s_6_14: Bits = Bits::new(s_6_13 as u128, 2u16);
        // D s_6_15: cast zx s_6_14 -> i
        let s_6_15: i128 = (s_6_14.value() as i128);
        // D s_6_16: cast reint s_6_15 -> i64
        let s_6_16: i64 = (s_6_15 as i64);
        // C s_6_17: const #440u : u32
        let s_6_17: u32 = 440;
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
        // D s_6_22: cast zx s_6_16 -> i
        let s_6_22: i128 = (i128::try_from(s_6_16).unwrap());
        // D s_6_23: cast zx s_6_21 -> i
        let s_6_23: i128 = (i128::try_from(s_6_21).unwrap());
        // D s_6_24: cmp-gt s_6_22 s_6_23
        let s_6_24: bool = ((s_6_22) > (s_6_23));
        // N s_6_25: branch s_6_24 b10 b7
        if s_6_24 {
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
        // D s_11_7: write-var gs#10152 <= s_11_6
        fn_state.gs_10152 = s_11_6;
        // N s_11_8: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var except:struct
        let s_12_0: ProductTypeb7f99f96751e17c4 = fn_state.except;
        // D s_12_1: write-var except <= s_12_0
        fn_state.except = s_12_0;
        // D s_12_2: read-var except:struct
        let s_12_2: ProductTypeb7f99f96751e17c4 = fn_state.except;
        // D s_12_3: write-var except <= s_12_2
        fn_state.except = s_12_2;
        // N s_12_4: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_13_0: const #"vector instructions set TFV to 1" : str
        let s_13_0: &'static str = "vector instructions set TFV to 1";
        // S s_13_1: call __IMPDEF_boolean(s_13_0)
        let s_13_1: bool = u__IMPDEF_boolean(state, tracer, s_13_0);
        // N s_13_2: branch s_13_1 b15 b14
        if s_13_1 {
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
        // C s_14_0: const #0u : u8
        let s_14_0: bool = false;
        // S s_14_1: call Bit(s_14_0)
        let s_14_1: bool = Bit(state, tracer, s_14_0);
        // D s_14_2: read-var except:struct
        let s_14_2: ProductTypeb7f99f96751e17c4 = fn_state.except;
        // D s_14_3: write-var except <= s_14_2
        fn_state.except = s_14_2;
        // N s_14_4: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_15_0: const #1u : u8
        let s_15_0: bool = true;
        // S s_15_1: call Bit(s_15_0)
        let s_15_1: bool = Bit(state, tracer, s_15_0);
        // D s_15_2: read-var except:struct
        let s_15_2: ProductTypeb7f99f96751e17c4 = fn_state.except;
        // D s_15_3: write-var except <= s_15_2
        fn_state.except = s_15_2;
        // N s_15_4: jump b2
        return block_2(state, tracer, fn_state);
    }
}
