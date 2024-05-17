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
pub fn MismatchedMemCpyException<T: Tracer>(
    state: &mut State,
    tracer: &T,
    option_a_name: bool,
    destreg: i128,
    srcreg: i128,
    sizereg: i128,
    wrong_option: bool,
    from_epilogue: bool,
    options_name: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        except: ProductTypeb7f99f96751e17c4,
        preferred_exception_return: u64,
        vect_offset: i64,
        ga_18083: bool,
        ga_18091: bool,
        ga_18087: bool,
        target_el: u8,
        option_a_name: bool,
        destreg: i128,
        srcreg: i128,
        sizereg: i128,
        wrong_option: bool,
        from_epilogue: bool,
        options_name: u8,
    }
    let fn_state = FunctionState {
        option_a_name,
        destreg,
        srcreg,
        sizereg,
        wrong_option,
        from_epilogue,
        options_name,
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
        // C s_0_16: const #0u : u8
        let s_0_16: bool = false;
        // S s_0_17: call Bit(s_0_16)
        let s_0_17: bool = Bit(state, tracer, s_0_16);
        // D s_0_18: read-var except:struct
        let s_0_18: ProductTypeb7f99f96751e17c4 = fn_state.except;
        // D s_0_19: write-var except <= s_0_18
        fn_state.except = s_0_18;
        // C s_0_20: const #0u : u8
        let s_0_20: bool = false;
        // S s_0_21: call Bit(s_0_20)
        let s_0_21: bool = Bit(state, tracer, s_0_20);
        // D s_0_22: read-var except:struct
        let s_0_22: ProductTypeb7f99f96751e17c4 = fn_state.except;
        // D s_0_23: write-var except <= s_0_22
        fn_state.except = s_0_22;
        // D s_0_24: read-var except:struct
        let s_0_24: ProductTypeb7f99f96751e17c4 = fn_state.except;
        // D s_0_25: write-var except <= s_0_24
        fn_state.except = s_0_24;
        // D s_0_26: read-var from_epilogue:u8
        let s_0_26: bool = fn_state.from_epilogue;
        // N s_0_27: branch s_0_26 b9 b1
        if s_0_26 {
            return block_9(state, tracer, fn_state);
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
        // D s_1_1: write-var ga#18083 <= s_1_0
        fn_state.ga_18083 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var ga#18083:u8
        let s_2_0: bool = fn_state.ga_18083;
        // D s_2_1: call Bit(s_2_0)
        let s_2_1: bool = Bit(state, tracer, s_2_0);
        // D s_2_2: read-var except:struct
        let s_2_2: ProductTypeb7f99f96751e17c4 = fn_state.except;
        // D s_2_3: write-var except <= s_2_2
        fn_state.except = s_2_2;
        // D s_2_4: read-var wrong_option:u8
        let s_2_4: bool = fn_state.wrong_option;
        // N s_2_5: branch s_2_4 b8 b3
        if s_2_4 {
            return block_8(state, tracer, fn_state);
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
        // D s_3_1: write-var ga#18087 <= s_3_0
        fn_state.ga_18087 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var ga#18087:u8
        let s_4_0: bool = fn_state.ga_18087;
        // D s_4_1: call Bit(s_4_0)
        let s_4_1: bool = Bit(state, tracer, s_4_0);
        // D s_4_2: read-var except:struct
        let s_4_2: ProductTypeb7f99f96751e17c4 = fn_state.except;
        // D s_4_3: write-var except <= s_4_2
        fn_state.except = s_4_2;
        // D s_4_4: read-var option_a_name:u8
        let s_4_4: bool = fn_state.option_a_name;
        // N s_4_5: branch s_4_4 b7 b5
        if s_4_4 {
            return block_7(state, tracer, fn_state);
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
        // D s_5_1: write-var ga#18091 <= s_5_0
        fn_state.ga_18091 = s_5_0;
        // N s_5_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var ga#18091:u8
        let s_6_0: bool = fn_state.ga_18091;
        // D s_6_1: call Bit(s_6_0)
        let s_6_1: bool = Bit(state, tracer, s_6_0);
        // D s_6_2: read-var except:struct
        let s_6_2: ProductTypeb7f99f96751e17c4 = fn_state.except;
        // D s_6_3: write-var except <= s_6_2
        fn_state.except = s_6_2;
        // C s_6_4: const #4s : i
        let s_6_4: i128 = 4;
        // C s_6_5: const #0s : i
        let s_6_5: i128 = 0;
        // D s_6_6: read-var destreg:i
        let s_6_6: i128 = fn_state.destreg;
        // D s_6_7: call integer_subrange(s_6_6, s_6_4, s_6_5)
        let s_6_7: Bits = integer_subrange(state, tracer, s_6_6, s_6_4, s_6_5);
        // D s_6_8: read-var except:struct
        let s_6_8: ProductTypeb7f99f96751e17c4 = fn_state.except;
        // D s_6_9: write-var except <= s_6_8
        fn_state.except = s_6_8;
        // C s_6_10: const #4s : i
        let s_6_10: i128 = 4;
        // C s_6_11: const #0s : i
        let s_6_11: i128 = 0;
        // D s_6_12: read-var srcreg:i
        let s_6_12: i128 = fn_state.srcreg;
        // D s_6_13: call integer_subrange(s_6_12, s_6_10, s_6_11)
        let s_6_13: Bits = integer_subrange(state, tracer, s_6_12, s_6_10, s_6_11);
        // D s_6_14: read-var except:struct
        let s_6_14: ProductTypeb7f99f96751e17c4 = fn_state.except;
        // D s_6_15: write-var except <= s_6_14
        fn_state.except = s_6_14;
        // C s_6_16: const #4s : i
        let s_6_16: i128 = 4;
        // C s_6_17: const #0s : i
        let s_6_17: i128 = 0;
        // D s_6_18: read-var sizereg:i
        let s_6_18: i128 = fn_state.sizereg;
        // D s_6_19: call integer_subrange(s_6_18, s_6_16, s_6_17)
        let s_6_19: Bits = integer_subrange(state, tracer, s_6_18, s_6_16, s_6_17);
        // D s_6_20: read-var except:struct
        let s_6_20: ProductTypeb7f99f96751e17c4 = fn_state.except;
        // D s_6_21: write-var except <= s_6_20
        fn_state.except = s_6_20;
        // D s_6_22: read-var vect_offset:i64
        let s_6_22: i64 = fn_state.vect_offset;
        // D s_6_23: cast zx s_6_22 -> i
        let s_6_23: i128 = (i128::try_from(s_6_22).unwrap());
        // D s_6_24: read-var target_el:u8
        let s_6_24: u8 = fn_state.target_el;
        // D s_6_25: read-var except:struct
        let s_6_25: ProductTypeb7f99f96751e17c4 = fn_state.except;
        // D s_6_26: read-var preferred_exception_return:u64
        let s_6_26: u64 = fn_state.preferred_exception_return;
        // D s_6_27: call AArch64_TakeException(s_6_24, s_6_25, s_6_26, s_6_23)
        let s_6_27: () = AArch64_TakeException(
            state,
            tracer,
            s_6_24,
            s_6_25,
            s_6_26,
            s_6_23,
        );
        // N s_6_28: return
        return;
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_7_0: const #1u : u8
        let s_7_0: bool = true;
        // D s_7_1: write-var ga#18091 <= s_7_0
        fn_state.ga_18091 = s_7_0;
        // N s_7_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_8_0: const #1u : u8
        let s_8_0: bool = true;
        // D s_8_1: write-var ga#18087 <= s_8_0
        fn_state.ga_18087 = s_8_0;
        // N s_8_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_9_0: const #1u : u8
        let s_9_0: bool = true;
        // D s_9_1: write-var ga#18083 <= s_9_0
        fn_state.ga_18083 = s_9_0;
        // N s_9_2: jump b2
        return block_2(state, tracer, fn_state);
    }
}
