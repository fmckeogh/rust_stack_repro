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
use CurrentVL_read::*;
use u__id::*;
use set_subrange_zeros::*;
use fdiv_int::*;
use common::*;
pub fn ElemFFR_set<T: Tracer>(
    state: &mut State,
    tracer: &T,
    e: i128,
    esize: i128,
    value_name: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_21528: bool,
        VL: i64,
        n: i128,
        psize: i128,
        e: i128,
        esize: i128,
        value_name: bool,
    }
    let fn_state = FunctionState {
        e,
        esize,
        value_name,
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
        // S s_0_1: call CurrentVL_read(s_0_0)
        let s_0_1: i64 = CurrentVL_read(state, tracer, s_0_0);
        // D s_0_2: write-var VL <= s_0_1
        fn_state.VL = s_0_1;
        // C s_0_3: const #8s : i
        let s_0_3: i128 = 8;
        // D s_0_4: read-var esize:i
        let s_0_4: i128 = fn_state.esize;
        // D s_0_5: call fdiv_int(s_0_4, s_0_3)
        let s_0_5: i128 = fdiv_int(state, tracer, s_0_4, s_0_3);
        // D s_0_6: write-var psize <= s_0_5
        fn_state.psize = s_0_5;
        // D s_0_7: read-var e:i
        let s_0_7: i128 = fn_state.e;
        // D s_0_8: read-var psize:i
        let s_0_8: i128 = fn_state.psize;
        // D s_0_9: mul s_0_7 s_0_8
        let s_0_9: i128 = ((s_0_7) * (s_0_8));
        // D s_0_10: write-var n <= s_0_9
        fn_state.n = s_0_9;
        // C s_0_11: const #0s : i
        let s_0_11: i128 = 0;
        // D s_0_12: read-var n:i
        let s_0_12: i128 = fn_state.n;
        // D s_0_13: cmp-ge s_0_12 s_0_11
        let s_0_13: bool = ((s_0_12) >= (s_0_11));
        // N s_0_14: branch s_0_13 b3 b1
        if s_0_13 {
            return block_3(state, tracer, fn_state);
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
        // D s_1_1: write-var gs#21528 <= s_1_0
        fn_state.gs_21528 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var gs#21528:u8
        let s_2_0: bool = fn_state.gs_21528;
        // N s_2_1: assert s_2_0
        let s_2_1: () = assert!(s_2_0);
        // D s_2_2: read-var n:i
        let s_2_2: i128 = fn_state.n;
        // D s_2_3: call __id(s_2_2)
        let s_2_3: i128 = u__id(state, tracer, s_2_2);
        // D s_2_4: read-var n:i
        let s_2_4: i128 = fn_state.n;
        // D s_2_5: call __id(s_2_4)
        let s_2_5: i128 = u__id(state, tracer, s_2_4);
        // D s_2_6: read-var psize:i
        let s_2_6: i128 = fn_state.psize;
        // D s_2_7: call __id(s_2_6)
        let s_2_7: i128 = u__id(state, tracer, s_2_6);
        // D s_2_8: add s_2_5 s_2_7
        let s_2_8: i128 = (s_2_5 + s_2_7);
        // C s_2_9: const #1s : i
        let s_2_9: i128 = 1;
        // D s_2_10: sub s_2_8 s_2_9
        let s_2_10: i128 = ((s_2_8) - (s_2_9));
        // D s_2_11: cmp-le s_2_3 s_2_10
        let s_2_11: bool = ((s_2_3) <= (s_2_10));
        // N s_2_12: assert s_2_11
        let s_2_12: () = assert!(s_2_11);
        // D s_2_13: read-var n:i
        let s_2_13: i128 = fn_state.n;
        // D s_2_14: read-var psize:i
        let s_2_14: i128 = fn_state.psize;
        // D s_2_15: add s_2_13 s_2_14
        let s_2_15: i128 = (s_2_13 + s_2_14);
        // D s_2_16: cast reint s_2_15 -> i64
        let s_2_16: i64 = (s_2_15 as i64);
        // C s_2_17: const #1s : i
        let s_2_17: i128 = 1;
        // D s_2_18: cast zx s_2_16 -> i
        let s_2_18: i128 = (i128::try_from(s_2_16).unwrap());
        // D s_2_19: sub s_2_18 s_2_17
        let s_2_19: i128 = ((s_2_18) - (s_2_17));
        // D s_2_20: cast reint s_2_19 -> i64
        let s_2_20: i64 = (s_2_19 as i64);
        // D s_2_21: read-var value_name:u8
        let s_2_21: bool = fn_state.value_name;
        // D s_2_22: cast zx s_2_21 -> bv
        let s_2_22: Bits = Bits::new(s_2_21 as u128, 1u16);
        // D s_2_23: size-of s_2_22
        let s_2_23: u16 = s_2_22.length();
        // D s_2_24: cast zx s_2_23 -> i
        let s_2_24: i128 = (i128::try_from(s_2_23).unwrap());
        // D s_2_25: cast reint s_2_24 -> i64
        let s_2_25: i64 = (s_2_24 as i64);
        // D s_2_26: cast zx s_2_25 -> i
        let s_2_26: i128 = (i128::try_from(s_2_25).unwrap());
        // D s_2_27: read-var n:i
        let s_2_27: i128 = fn_state.n;
        // D s_2_28: add s_2_27 s_2_26
        let s_2_28: i128 = (s_2_27 + s_2_26);
        // D s_2_29: cast reint s_2_28 -> i64
        let s_2_29: i64 = (s_2_28 as i64);
        // C s_2_30: const #256s : i
        let s_2_30: i128 = 256;
        // C s_2_31: const #14552u : u32
        let s_2_31: u32 = 14552;
        // D s_2_32: read-reg s_2_31:u256
        let s_2_32: u64 = {
            let value = state.read_register::<u64>(s_2_31 as isize);
            tracer.read_register(s_2_31 as isize, value);
            value
        };
        // D s_2_33: cast zx s_2_32 -> bv
        let s_2_33: Bits = Bits::new(s_2_32 as u128, 256u16);
        // D s_2_34: cast zx s_2_20 -> i
        let s_2_34: i128 = (i128::try_from(s_2_20).unwrap());
        // D s_2_35: cast zx s_2_29 -> i
        let s_2_35: i128 = (i128::try_from(s_2_29).unwrap());
        // D s_2_36: call set_subrange_zeros(s_2_30, s_2_33, s_2_34, s_2_35)
        let s_2_36: Bits = set_subrange_zeros(
            state,
            tracer,
            s_2_30,
            s_2_33,
            s_2_34,
            s_2_35,
        );
        // D s_2_37: cast reint s_2_36 -> u256
        let s_2_37: u64 = (s_2_36.value() as u64);
        // C s_2_38: const #14552u : u32
        let s_2_38: u32 = 14552;
        // N s_2_39: write-reg s_2_38 <= s_2_37
        let s_2_39: () = {
            state.write_register::<u64>(s_2_38 as isize, s_2_37);
            tracer.write_register(s_2_38 as isize, s_2_37);
        };
        // D s_2_40: read-var value_name:u8
        let s_2_40: bool = fn_state.value_name;
        // D s_2_41: cast zx s_2_40 -> bv
        let s_2_41: Bits = Bits::new(s_2_40 as u128, 1u16);
        // D s_2_42: size-of s_2_41
        let s_2_42: u16 = s_2_41.length();
        // D s_2_43: cast zx s_2_42 -> i
        let s_2_43: i128 = (i128::try_from(s_2_42).unwrap());
        // D s_2_44: cast reint s_2_43 -> i64
        let s_2_44: i64 = (s_2_43 as i64);
        // D s_2_45: cast zx s_2_44 -> i
        let s_2_45: i128 = (i128::try_from(s_2_44).unwrap());
        // D s_2_46: read-var n:i
        let s_2_46: i128 = fn_state.n;
        // D s_2_47: add s_2_46 s_2_45
        let s_2_47: i128 = (s_2_46 + s_2_45);
        // D s_2_48: cast reint s_2_47 -> i64
        let s_2_48: i64 = (s_2_47 as i64);
        // C s_2_49: const #1s : i
        let s_2_49: i128 = 1;
        // D s_2_50: cast zx s_2_48 -> i
        let s_2_50: i128 = (i128::try_from(s_2_48).unwrap());
        // D s_2_51: sub s_2_50 s_2_49
        let s_2_51: i128 = ((s_2_50) - (s_2_49));
        // D s_2_52: cast reint s_2_51 -> i64
        let s_2_52: i64 = (s_2_51 as i64);
        // C s_2_53: const #14552u : u32
        let s_2_53: u32 = 14552;
        // D s_2_54: read-reg s_2_53:u256
        let s_2_54: u64 = {
            let value = state.read_register::<u64>(s_2_53 as isize);
            tracer.read_register(s_2_53 as isize, value);
            value
        };
        // D s_2_55: cast zx s_2_54 -> bv
        let s_2_55: Bits = Bits::new(s_2_54 as u128, 256u16);
        // D s_2_56: cast zx s_2_52 -> i
        let s_2_56: i128 = (i128::try_from(s_2_52).unwrap());
        // D s_2_57: read-var value_name:u8
        let s_2_57: bool = fn_state.value_name;
        // D s_2_58: cast zx s_2_57 -> bv
        let s_2_58: Bits = Bits::new(s_2_57 as u128, 1u16);
        // D s_2_59: read-var n:i
        let s_2_59: i128 = fn_state.n;
        // D s_2_60: sub s_2_56 s_2_59
        let s_2_60: i128 = ((s_2_56) - (s_2_59));
        // C s_2_61: const #1u : u64
        let s_2_61: u64 = 1;
        // C s_2_62: cast zx s_2_61 -> bv
        let s_2_62: Bits = Bits::new(s_2_61 as u128, 64u16);
        // D s_2_63: lsl s_2_62 s_2_60
        let s_2_63: Bits = s_2_62 << s_2_60;
        // D s_2_64: sub s_2_63 s_2_62
        let s_2_64: Bits = ((s_2_63) - (s_2_62));
        // D s_2_65: and s_2_58 s_2_64
        let s_2_65: Bits = ((s_2_58) & (s_2_64));
        // D s_2_66: lsl s_2_65 s_2_59
        let s_2_66: Bits = s_2_65 << s_2_59;
        // D s_2_67: lsl s_2_64 s_2_59
        let s_2_67: Bits = s_2_64 << s_2_59;
        // D s_2_68: cmpl s_2_67
        let s_2_68: Bits = !s_2_67;
        // D s_2_69: and s_2_55 s_2_68
        let s_2_69: Bits = ((s_2_55) & (s_2_68));
        // D s_2_70: or s_2_69 s_2_66
        let s_2_70: Bits = ((s_2_69) | (s_2_66));
        // D s_2_71: cast reint s_2_70 -> u256
        let s_2_71: u64 = (s_2_70.value() as u64);
        // C s_2_72: const #14552u : u32
        let s_2_72: u32 = 14552;
        // N s_2_73: write-reg s_2_72 <= s_2_71
        let s_2_73: () = {
            state.write_register::<u64>(s_2_72 as isize, s_2_71);
            tracer.write_register(s_2_72 as isize, s_2_71);
        };
        // N s_2_74: return
        return;
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var n:i
        let s_3_0: i128 = fn_state.n;
        // D s_3_1: read-var psize:i
        let s_3_1: i128 = fn_state.psize;
        // D s_3_2: add s_3_0 s_3_1
        let s_3_2: i128 = (s_3_0 + s_3_1);
        // C s_3_3: const #8s : i
        let s_3_3: i128 = 8;
        // D s_3_4: read-var VL:i64
        let s_3_4: i64 = fn_state.VL;
        // D s_3_5: cast zx s_3_4 -> i
        let s_3_5: i128 = (i128::try_from(s_3_4).unwrap());
        // D s_3_6: div s_3_5 s_3_3
        let s_3_6: i128 = ((s_3_5) / (s_3_3));
        // D s_3_7: cast reint s_3_6 -> i64
        let s_3_7: i64 = (s_3_6 as i64);
        // D s_3_8: cast zx s_3_7 -> i
        let s_3_8: i128 = (i128::try_from(s_3_7).unwrap());
        // D s_3_9: cmp-le s_3_2 s_3_8
        let s_3_9: bool = ((s_3_2) <= (s_3_8));
        // D s_3_10: write-var gs#21528 <= s_3_9
        fn_state.gs_21528 = s_3_9;
        // N s_3_11: jump b2
        return block_2(state, tracer, fn_state);
    }
}
