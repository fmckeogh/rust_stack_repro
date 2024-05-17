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
use MismatchedCpySetTargetEL::*;
use Bit::*;
use AArch64_TakeException::*;
use ExceptionSyndrome::*;
use integer_subrange::*;
use common::*;
pub fn MismatchedMemSetException<T: Tracer>(
    state: &mut State,
    tracer: &T,
    option_a_name: bool,
    destreg: i128,
    datareg: i128,
    sizereg: i128,
    wrong_option: bool,
    from_epilogue: bool,
    options_name: u8,
    is_SETG: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        ga_18115: bool,
        except: ProductTypeb7f99f96751e17c4,
        ga_18119: bool,
        preferred_exception_return: u64,
        vect_offset: i64,
        ga_18109: bool,
        target_el: u8,
        ga_18123: bool,
        option_a_name: bool,
        destreg: i128,
        datareg: i128,
        sizereg: i128,
        wrong_option: bool,
        from_epilogue: bool,
        options_name: u8,
        is_SETG: bool,
    }
    let fn_state = FunctionState {
        option_a_name,
        destreg,
        datareg,
        sizereg,
        wrong_option,
        from_epilogue,
        options_name,
        is_SETG,
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
        // C s_0_10: const #() : ()
        let s_0_10: () = ();
        // S s_0_11: call MismatchedCpySetTargetEL(s_0_10)
        let s_0_11: u8 = MismatchedCpySetTargetEL(state, tracer, s_0_10);
        // D s_0_12: write-var target_el <= s_0_11
        fn_state.target_el = s_0_11;
        // C s_0_13: const #37u : u32
        let s_0_13: u32 = 37;
        // S s_0_14: call ExceptionSyndrome(s_0_13)
        let s_0_14: ProductTypeb7f99f96751e17c4 = ExceptionSyndrome(
            state,
            tracer,
            s_0_13,
        );
        // D s_0_15: write-var except <= s_0_14
        fn_state.except = s_0_14;
        // C s_0_16: const #1u : u8
        let s_0_16: bool = true;
        // S s_0_17: call Bit(s_0_16)
        let s_0_17: bool = Bit(state, tracer, s_0_16);
        // D s_0_18: read-var except:struct
        let s_0_18: ProductTypeb7f99f96751e17c4 = fn_state.except;
        // D s_0_19: write-var except <= s_0_18
        fn_state.except = s_0_18;
        // D s_0_20: read-var is_SETG:u8
        let s_0_20: bool = fn_state.is_SETG;
        // N s_0_21: branch s_0_20 b12 b1
        if s_0_20 {
            return block_12(state, tracer, fn_state);
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
        // D s_1_1: write-var ga#18109 <= s_1_0
        fn_state.ga_18109 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var ga#18109:u8
        let s_2_0: bool = fn_state.ga_18109;
        // D s_2_1: call Bit(s_2_0)
        let s_2_1: bool = Bit(state, tracer, s_2_0);
        // D s_2_2: read-var except:struct
        let s_2_2: ProductTypeb7f99f96751e17c4 = fn_state.except;
        // D s_2_3: write-var except <= s_2_2
        fn_state.except = s_2_2;
        // D s_2_4: read-var except:struct
        let s_2_4: ProductTypeb7f99f96751e17c4 = fn_state.except;
        // D s_2_5: write-var except <= s_2_4
        fn_state.except = s_2_4;
        // D s_2_6: read-var from_epilogue:u8
        let s_2_6: bool = fn_state.from_epilogue;
        // N s_2_7: branch s_2_6 b11 b3
        if s_2_6 {
            return block_11(state, tracer, fn_state);
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
        // D s_3_1: write-var ga#18115 <= s_3_0
        fn_state.ga_18115 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var ga#18115:u8
        let s_4_0: bool = fn_state.ga_18115;
        // D s_4_1: call Bit(s_4_0)
        let s_4_1: bool = Bit(state, tracer, s_4_0);
        // D s_4_2: read-var except:struct
        let s_4_2: ProductTypeb7f99f96751e17c4 = fn_state.except;
        // D s_4_3: write-var except <= s_4_2
        fn_state.except = s_4_2;
        // D s_4_4: read-var wrong_option:u8
        let s_4_4: bool = fn_state.wrong_option;
        // N s_4_5: branch s_4_4 b10 b5
        if s_4_4 {
            return block_10(state, tracer, fn_state);
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
        // D s_5_1: write-var ga#18119 <= s_5_0
        fn_state.ga_18119 = s_5_0;
        // N s_5_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var ga#18119:u8
        let s_6_0: bool = fn_state.ga_18119;
        // D s_6_1: call Bit(s_6_0)
        let s_6_1: bool = Bit(state, tracer, s_6_0);
        // D s_6_2: read-var except:struct
        let s_6_2: ProductTypeb7f99f96751e17c4 = fn_state.except;
        // D s_6_3: write-var except <= s_6_2
        fn_state.except = s_6_2;
        // D s_6_4: read-var option_a_name:u8
        let s_6_4: bool = fn_state.option_a_name;
        // N s_6_5: branch s_6_4 b9 b7
        if s_6_4 {
            return block_9(state, tracer, fn_state);
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
        // D s_7_1: write-var ga#18123 <= s_7_0
        fn_state.ga_18123 = s_7_0;
        // N s_7_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var ga#18123:u8
        let s_8_0: bool = fn_state.ga_18123;
        // D s_8_1: call Bit(s_8_0)
        let s_8_1: bool = Bit(state, tracer, s_8_0);
        // D s_8_2: read-var except:struct
        let s_8_2: ProductTypeb7f99f96751e17c4 = fn_state.except;
        // D s_8_3: write-var except <= s_8_2
        fn_state.except = s_8_2;
        // C s_8_4: const #4s : i
        let s_8_4: i128 = 4;
        // C s_8_5: const #0s : i
        let s_8_5: i128 = 0;
        // D s_8_6: read-var destreg:i
        let s_8_6: i128 = fn_state.destreg;
        // D s_8_7: call integer_subrange(s_8_6, s_8_4, s_8_5)
        let s_8_7: Bits = integer_subrange(state, tracer, s_8_6, s_8_4, s_8_5);
        // D s_8_8: read-var except:struct
        let s_8_8: ProductTypeb7f99f96751e17c4 = fn_state.except;
        // D s_8_9: write-var except <= s_8_8
        fn_state.except = s_8_8;
        // C s_8_10: const #4s : i
        let s_8_10: i128 = 4;
        // C s_8_11: const #0s : i
        let s_8_11: i128 = 0;
        // D s_8_12: read-var datareg:i
        let s_8_12: i128 = fn_state.datareg;
        // D s_8_13: call integer_subrange(s_8_12, s_8_10, s_8_11)
        let s_8_13: Bits = integer_subrange(state, tracer, s_8_12, s_8_10, s_8_11);
        // D s_8_14: read-var except:struct
        let s_8_14: ProductTypeb7f99f96751e17c4 = fn_state.except;
        // D s_8_15: write-var except <= s_8_14
        fn_state.except = s_8_14;
        // C s_8_16: const #4s : i
        let s_8_16: i128 = 4;
        // C s_8_17: const #0s : i
        let s_8_17: i128 = 0;
        // D s_8_18: read-var sizereg:i
        let s_8_18: i128 = fn_state.sizereg;
        // D s_8_19: call integer_subrange(s_8_18, s_8_16, s_8_17)
        let s_8_19: Bits = integer_subrange(state, tracer, s_8_18, s_8_16, s_8_17);
        // D s_8_20: read-var except:struct
        let s_8_20: ProductTypeb7f99f96751e17c4 = fn_state.except;
        // D s_8_21: write-var except <= s_8_20
        fn_state.except = s_8_20;
        // D s_8_22: read-var vect_offset:i64
        let s_8_22: i64 = fn_state.vect_offset;
        // D s_8_23: cast zx s_8_22 -> i
        let s_8_23: i128 = (i128::try_from(s_8_22).unwrap());
        // D s_8_24: read-var target_el:u8
        let s_8_24: u8 = fn_state.target_el;
        // D s_8_25: read-var except:struct
        let s_8_25: ProductTypeb7f99f96751e17c4 = fn_state.except;
        // D s_8_26: read-var preferred_exception_return:u64
        let s_8_26: u64 = fn_state.preferred_exception_return;
        // D s_8_27: call AArch64_TakeException(s_8_24, s_8_25, s_8_26, s_8_23)
        let s_8_27: () = AArch64_TakeException(
            state,
            tracer,
            s_8_24,
            s_8_25,
            s_8_26,
            s_8_23,
        );
        // N s_8_28: return
        return;
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_9_0: const #1u : u8
        let s_9_0: bool = true;
        // D s_9_1: write-var ga#18123 <= s_9_0
        fn_state.ga_18123 = s_9_0;
        // N s_9_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_10_0: const #1u : u8
        let s_10_0: bool = true;
        // D s_10_1: write-var ga#18119 <= s_10_0
        fn_state.ga_18119 = s_10_0;
        // N s_10_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_11_0: const #1u : u8
        let s_11_0: bool = true;
        // D s_11_1: write-var ga#18115 <= s_11_0
        fn_state.ga_18115 = s_11_0;
        // N s_11_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_12_0: const #1u : u8
        let s_12_0: bool = true;
        // D s_12_1: write-var ga#18109 <= s_12_0
        fn_state.ga_18109 = s_12_0;
        // N s_12_2: jump b2
        return block_2(state, tracer, fn_state);
    }
}
