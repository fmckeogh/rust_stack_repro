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
use ThisInstr::*;
use Zeros::*;
use AArch64_TakeException::*;
use ExceptionSyndrome::*;
use u_get_HCR_EL2_Type_TGE::*;
use common::*;
pub fn GCSDataCheckException<T: Tracer>(
    state: &mut State,
    tracer: &T,
    gcsinst_type: u32,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        rn_unknown: bool,
        target_el: u8,
        except: ProductTypeb7f99f96751e17c4,
        preferred_exception_return: u64,
        is_ret: bool,
        vect_offset: i64,
        gcsinst_type: u32,
    }
    let fn_state = FunctionState {
        gcsinst_type,
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
        // C s_0_10: const #0u : u8
        let s_0_10: bool = false;
        // D s_0_11: write-var rn_unknown <= s_0_10
        fn_state.rn_unknown = s_0_10;
        // C s_0_12: const #0u : u8
        let s_0_12: bool = false;
        // D s_0_13: write-var is_ret <= s_0_12
        fn_state.is_ret = s_0_12;
        // C s_0_14: const #16975u : u32
        let s_0_14: u32 = 16975;
        // D s_0_15: read-reg s_0_14:u8
        let s_0_15: u8 = {
            let value = state.read_register::<u8>(s_0_14 as isize);
            tracer.read_register(s_0_14 as isize, value);
            value
        };
        // D s_0_16: cast zx s_0_15 -> bv
        let s_0_16: Bits = Bits::new(s_0_15 as u128, 2u16);
        // C s_0_17: const #448u : u32
        let s_0_17: u32 = 448;
        // D s_0_18: read-reg s_0_17:u8
        let s_0_18: u8 = {
            let value = state.read_register::<u8>(s_0_17 as isize);
            tracer.read_register(s_0_17 as isize, value);
            value
        };
        // D s_0_19: cast zx s_0_18 -> bv
        let s_0_19: Bits = Bits::new(s_0_18 as u128, 2u16);
        // D s_0_20: cmp-eq s_0_16 s_0_19
        let s_0_20: bool = ((s_0_16) == (s_0_19));
        // N s_0_21: branch s_0_20 b25 b1
        if s_0_20 {
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
        // C s_1_0: const #16975u : u32
        let s_1_0: u32 = 16975;
        // D s_1_1: read-reg s_1_0:u8
        let s_1_1: u8 = {
            let value = state.read_register::<u8>(s_1_0 as isize);
            tracer.read_register(s_1_0 as isize, value);
            value
        };
        // D s_1_2: write-var target_el <= s_1_1
        fn_state.target_el = s_1_1;
        // N s_1_3: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_2_0: const #38u : u32
        let s_2_0: u32 = 38;
        // S s_2_1: call ExceptionSyndrome(s_2_0)
        let s_2_1: ProductTypeb7f99f96751e17c4 = ExceptionSyndrome(state, tracer, s_2_0);
        // D s_2_2: write-var except <= s_2_1
        fn_state.except = s_2_1;
        // C s_2_3: const #0u : u32
        let s_2_3: u32 = 0;
        // D s_2_4: read-var gcsinst_type:u32
        let s_2_4: u32 = fn_state.gcsinst_type;
        // D s_2_5: cmp-eq s_2_3 s_2_4
        let s_2_5: bool = ((s_2_3) == (s_2_4));
        // D s_2_6: not s_2_5
        let s_2_6: bool = !s_2_5;
        // N s_2_7: branch s_2_6 b10 b3
        if s_2_6 {
            return block_10(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var except:struct
        let s_3_0: ProductTypeb7f99f96751e17c4 = fn_state.except;
        // D s_3_1: write-var except <= s_3_0
        fn_state.except = s_3_0;
        // C s_3_2: const #1u : u8
        let s_3_2: bool = true;
        // D s_3_3: write-var is_ret <= s_3_2
        fn_state.is_ret = s_3_2;
        // N s_3_4: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var rn_unknown:u8
        let s_4_0: bool = fn_state.rn_unknown;
        // C s_4_1: const #1u : u8
        let s_4_1: bool = true;
        // D s_4_2: cmp-eq s_4_0 s_4_1
        let s_4_2: bool = ((s_4_0) == (s_4_1));
        // N s_4_3: branch s_4_2 b9 b5
        if s_4_2 {
            return block_9(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var is_ret:u8
        let s_5_0: bool = fn_state.is_ret;
        // C s_5_1: const #1u : u8
        let s_5_1: bool = true;
        // D s_5_2: cmp-eq s_5_0 s_5_1
        let s_5_2: bool = ((s_5_0) == (s_5_1));
        // N s_5_3: branch s_5_2 b8 b6
        if s_5_2 {
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
        // C s_6_0: const #() : ()
        let s_6_0: () = ();
        // S s_6_1: call ThisInstr(s_6_0)
        let s_6_1: u32 = ThisInstr(state, tracer, s_6_0);
        // D s_6_2: read-var except:struct
        let s_6_2: ProductTypeb7f99f96751e17c4 = fn_state.except;
        // D s_6_3: write-var except <= s_6_2
        fn_state.except = s_6_2;
        // N s_6_4: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_7_0: const #15s : i
        let s_7_0: i128 = 15;
        // S s_7_1: call Zeros(s_7_0)
        let s_7_1: Bits = Zeros(state, tracer, s_7_0);
        // D s_7_2: read-var except:struct
        let s_7_2: ProductTypeb7f99f96751e17c4 = fn_state.except;
        // D s_7_3: write-var except <= s_7_2
        fn_state.except = s_7_2;
        // C s_7_4: const #64s : i64
        let s_7_4: i64 = 64;
        // C s_7_5: cast zx s_7_4 -> i
        let s_7_5: i128 = (i128::try_from(s_7_4).unwrap());
        // S s_7_6: call __UNKNOWN_bits(s_7_5)
        let s_7_6: Bits = u__UNKNOWN_bits(state, tracer, s_7_5);
        // S s_7_7: cast reint s_7_6 -> u64
        let s_7_7: u64 = (s_7_6.value() as u64);
        // D s_7_8: write-var except.9 <= s_7_7
        fn_state.except._9 = s_7_7;
        // D s_7_9: read-var vect_offset:i64
        let s_7_9: i64 = fn_state.vect_offset;
        // D s_7_10: cast zx s_7_9 -> i
        let s_7_10: i128 = (i128::try_from(s_7_9).unwrap());
        // D s_7_11: read-var target_el:u8
        let s_7_11: u8 = fn_state.target_el;
        // D s_7_12: read-var except:struct
        let s_7_12: ProductTypeb7f99f96751e17c4 = fn_state.except;
        // D s_7_13: read-var preferred_exception_return:u64
        let s_7_13: u64 = fn_state.preferred_exception_return;
        // D s_7_14: call AArch64_TakeException(s_7_11, s_7_12, s_7_13, s_7_10)
        let s_7_14: () = AArch64_TakeException(
            state,
            tracer,
            s_7_11,
            s_7_12,
            s_7_13,
            s_7_10,
        );
        // N s_7_15: return
        return;
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_8_0: const #() : ()
        let s_8_0: () = ();
        // S s_8_1: call ThisInstr(s_8_0)
        let s_8_1: u32 = ThisInstr(state, tracer, s_8_0);
        // D s_8_2: read-var except:struct
        let s_8_2: ProductTypeb7f99f96751e17c4 = fn_state.except;
        // D s_8_3: write-var except <= s_8_2
        fn_state.except = s_8_2;
        // N s_8_4: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_9_0: const #5s : i64
        let s_9_0: i64 = 5;
        // C s_9_1: cast zx s_9_0 -> i
        let s_9_1: i128 = (i128::try_from(s_9_0).unwrap());
        // S s_9_2: call __UNKNOWN_bits(s_9_1)
        let s_9_2: Bits = u__UNKNOWN_bits(state, tracer, s_9_1);
        // D s_9_3: read-var except:struct
        let s_9_3: ProductTypeb7f99f96751e17c4 = fn_state.except;
        // D s_9_4: write-var except <= s_9_3
        fn_state.except = s_9_3;
        // N s_9_5: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_10_0: const #1u : u32
        let s_10_0: u32 = 1;
        // D s_10_1: read-var gcsinst_type:u32
        let s_10_1: u32 = fn_state.gcsinst_type;
        // D s_10_2: cmp-eq s_10_0 s_10_1
        let s_10_2: bool = ((s_10_0) == (s_10_1));
        // D s_10_3: not s_10_2
        let s_10_3: bool = !s_10_2;
        // N s_10_4: branch s_10_3 b12 b11
        if s_10_3 {
            return block_12(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var except:struct
        let s_11_0: ProductTypeb7f99f96751e17c4 = fn_state.except;
        // D s_11_1: write-var except <= s_11_0
        fn_state.except = s_11_0;
        // N s_11_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_12_0: const #2u : u32
        let s_12_0: u32 = 2;
        // D s_12_1: read-var gcsinst_type:u32
        let s_12_1: u32 = fn_state.gcsinst_type;
        // D s_12_2: cmp-eq s_12_0 s_12_1
        let s_12_2: bool = ((s_12_0) == (s_12_1));
        // D s_12_3: not s_12_2
        let s_12_3: bool = !s_12_2;
        // N s_12_4: branch s_12_3 b14 b13
        if s_12_3 {
            return block_14(state, tracer, fn_state);
        } else {
            return block_13(state, tracer, fn_state);
        };
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var except:struct
        let s_13_0: ProductTypeb7f99f96751e17c4 = fn_state.except;
        // D s_13_1: write-var except <= s_13_0
        fn_state.except = s_13_0;
        // C s_13_2: const #1u : u8
        let s_13_2: bool = true;
        // D s_13_3: write-var is_ret <= s_13_2
        fn_state.is_ret = s_13_2;
        // N s_13_4: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_14_0: const #3u : u32
        let s_14_0: u32 = 3;
        // D s_14_1: read-var gcsinst_type:u32
        let s_14_1: u32 = fn_state.gcsinst_type;
        // D s_14_2: cmp-eq s_14_0 s_14_1
        let s_14_2: bool = ((s_14_0) == (s_14_1));
        // D s_14_3: not s_14_2
        let s_14_3: bool = !s_14_2;
        // N s_14_4: branch s_14_3 b16 b15
        if s_14_3 {
            return block_16(state, tracer, fn_state);
        } else {
            return block_15(state, tracer, fn_state);
        };
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_15_0: read-var except:struct
        let s_15_0: ProductTypeb7f99f96751e17c4 = fn_state.except;
        // D s_15_1: write-var except <= s_15_0
        fn_state.except = s_15_0;
        // C s_15_2: const #1u : u8
        let s_15_2: bool = true;
        // D s_15_3: write-var is_ret <= s_15_2
        fn_state.is_ret = s_15_2;
        // N s_15_4: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_16_0: const #4u : u32
        let s_16_0: u32 = 4;
        // D s_16_1: read-var gcsinst_type:u32
        let s_16_1: u32 = fn_state.gcsinst_type;
        // D s_16_2: cmp-eq s_16_0 s_16_1
        let s_16_2: bool = ((s_16_0) == (s_16_1));
        // D s_16_3: not s_16_2
        let s_16_3: bool = !s_16_2;
        // N s_16_4: branch s_16_3 b18 b17
        if s_16_3 {
            return block_18(state, tracer, fn_state);
        } else {
            return block_17(state, tracer, fn_state);
        };
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_17_0: read-var except:struct
        let s_17_0: ProductTypeb7f99f96751e17c4 = fn_state.except;
        // D s_17_1: write-var except <= s_17_0
        fn_state.except = s_17_0;
        // N s_17_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_18_0: const #5u : u32
        let s_18_0: u32 = 5;
        // D s_18_1: read-var gcsinst_type:u32
        let s_18_1: u32 = fn_state.gcsinst_type;
        // D s_18_2: cmp-eq s_18_0 s_18_1
        let s_18_2: bool = ((s_18_0) == (s_18_1));
        // D s_18_3: not s_18_2
        let s_18_3: bool = !s_18_2;
        // N s_18_4: branch s_18_3 b20 b19
        if s_18_3 {
            return block_20(state, tracer, fn_state);
        } else {
            return block_19(state, tracer, fn_state);
        };
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_19_0: read-var except:struct
        let s_19_0: ProductTypeb7f99f96751e17c4 = fn_state.except;
        // D s_19_1: write-var except <= s_19_0
        fn_state.except = s_19_0;
        // C s_19_2: const #1u : u8
        let s_19_2: bool = true;
        // D s_19_3: write-var rn_unknown <= s_19_2
        fn_state.rn_unknown = s_19_2;
        // N s_19_4: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_20_0: const #6u : u32
        let s_20_0: u32 = 6;
        // D s_20_1: read-var gcsinst_type:u32
        let s_20_1: u32 = fn_state.gcsinst_type;
        // D s_20_2: cmp-eq s_20_0 s_20_1
        let s_20_2: bool = ((s_20_0) == (s_20_1));
        // D s_20_3: not s_20_2
        let s_20_3: bool = !s_20_2;
        // N s_20_4: branch s_20_3 b22 b21
        if s_20_3 {
            return block_22(state, tracer, fn_state);
        } else {
            return block_21(state, tracer, fn_state);
        };
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_21_0: const #1u : u8
        let s_21_0: bool = true;
        // D s_21_1: write-var rn_unknown <= s_21_0
        fn_state.rn_unknown = s_21_0;
        // D s_21_2: read-var except:struct
        let s_21_2: ProductTypeb7f99f96751e17c4 = fn_state.except;
        // D s_21_3: write-var except <= s_21_2
        fn_state.except = s_21_2;
        // N s_21_4: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_22_0: const #7u : u32
        let s_22_0: u32 = 7;
        // D s_22_1: read-var gcsinst_type:u32
        let s_22_1: u32 = fn_state.gcsinst_type;
        // D s_22_2: cmp-eq s_22_0 s_22_1
        let s_22_2: bool = ((s_22_0) == (s_22_1));
        // D s_22_3: not s_22_2
        let s_22_3: bool = !s_22_2;
        // N s_22_4: branch s_22_3 b24 b23
        if s_22_3 {
            return block_24(state, tracer, fn_state);
        } else {
            return block_23(state, tracer, fn_state);
        };
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_23_0: read-var except:struct
        let s_23_0: ProductTypeb7f99f96751e17c4 = fn_state.except;
        // D s_23_1: write-var except <= s_23_0
        fn_state.except = s_23_0;
        // N s_23_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_24_0: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_25_0: const #102552u : u32
        let s_25_0: u32 = 102552;
        // D s_25_1: read-reg s_25_0:struct
        let s_25_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_25_0 as isize);
            tracer.read_register(s_25_0 as isize, value);
            value
        };
        // D s_25_2: call _get_HCR_EL2_Type_TGE(s_25_1)
        let s_25_2: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_25_1);
        // D s_25_3: cast zx s_25_2 -> bv
        let s_25_3: Bits = Bits::new(s_25_2 as u128, 1u16);
        // C s_25_4: const #0u : u8
        let s_25_4: bool = false;
        // C s_25_5: cast zx s_25_4 -> bv
        let s_25_5: Bits = Bits::new(s_25_4 as u128, 1u16);
        // D s_25_6: cmp-eq s_25_3 s_25_5
        let s_25_6: bool = ((s_25_3) == (s_25_5));
        // N s_25_7: branch s_25_6 b28 b26
        if s_25_6 {
            return block_28(state, tracer, fn_state);
        } else {
            return block_26(state, tracer, fn_state);
        };
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_26_0: const #432u : u32
        let s_26_0: u32 = 432;
        // D s_26_1: read-reg s_26_0:u8
        let s_26_1: u8 = {
            let value = state.read_register::<u8>(s_26_0 as isize);
            tracer.read_register(s_26_0 as isize, value);
            value
        };
        // D s_26_2: write-var target_el <= s_26_1
        fn_state.target_el = s_26_1;
        // N s_26_3: jump b27
        return block_27(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_27_0: jump b2
        return block_2(state, tracer, fn_state);
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
        // N s_28_3: jump b27
        return block_27(state, tracer, fn_state);
    }
}
