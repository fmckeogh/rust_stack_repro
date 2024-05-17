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
use X_set::*;
use X_read::*;
use u__id::*;
use common::*;
pub fn execute_aarch64_instrs_integer_ins_ext_extract_immediate<T: Tracer>(
    state: &mut State,
    tracer: &T,
    d: i64,
    datasize: i64,
    lsb: i64,
    m: i64,
    n: i64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        d: i64,
        datasize: i64,
        lsb: i64,
        m: i64,
        n: i64,
    }
    let fn_state = FunctionState {
        d,
        datasize,
        lsb,
        m,
        n,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_0_0: read-var datasize:i64
        let s_0_0: i64 = fn_state.datasize;
        // D s_0_1: cast zx s_0_0 -> i
        let s_0_1: i128 = (i128::try_from(s_0_0).unwrap());
        // D s_0_2: cast reint s_0_1 -> i64
        let s_0_2: i64 = (s_0_1 as i64);
        // D s_0_3: cast zx s_0_2 -> i
        let s_0_3: i128 = (i128::try_from(s_0_2).unwrap());
        // D s_0_4: cast reint s_0_3 -> i64
        let s_0_4: i64 = (s_0_3 as i64);
        // D s_0_5: read-var n:i64
        let s_0_5: i64 = fn_state.n;
        // D s_0_6: cast zx s_0_5 -> i
        let s_0_6: i128 = (i128::try_from(s_0_5).unwrap());
        // D s_0_7: call X_read(s_0_6, s_0_4)
        let s_0_7: Bits = X_read(state, tracer, s_0_6, s_0_4);
        // D s_0_8: cast zx s_0_2 -> i
        let s_0_8: i128 = (i128::try_from(s_0_2).unwrap());
        // D s_0_9: cast reint s_0_8 -> i64
        let s_0_9: i64 = (s_0_8 as i64);
        // D s_0_10: read-var m:i64
        let s_0_10: i64 = fn_state.m;
        // D s_0_11: cast zx s_0_10 -> i
        let s_0_11: i128 = (i128::try_from(s_0_10).unwrap());
        // D s_0_12: call X_read(s_0_11, s_0_9)
        let s_0_12: Bits = X_read(state, tracer, s_0_11, s_0_9);
        // D s_0_13: cast reint s_0_7 -> u128
        let s_0_13: u128 = (s_0_7.value() as u128);
        // D s_0_14: size-of s_0_7
        let s_0_14: u16 = s_0_7.length();
        // D s_0_15: cast reint s_0_12 -> u128
        let s_0_15: u128 = (s_0_12.value() as u128);
        // D s_0_16: size-of s_0_12
        let s_0_16: u16 = s_0_12.length();
        // D s_0_17: lsl s_0_13 s_0_16
        let s_0_17: u128 = s_0_13 << s_0_16;
        // D s_0_18: or s_0_17 s_0_15
        let s_0_18: u128 = ((s_0_17) | (s_0_15));
        // D s_0_19: add s_0_14 s_0_16
        let s_0_19: u16 = (s_0_14 + s_0_16);
        // D s_0_20: create-bits s_0_18 s_0_19
        let s_0_20: Bits = Bits::new(s_0_18, s_0_19);
        // D s_0_21: read-var lsb:i64
        let s_0_21: i64 = fn_state.lsb;
        // D s_0_22: cast zx s_0_21 -> i
        let s_0_22: i128 = (i128::try_from(s_0_21).unwrap());
        // D s_0_23: call __id(s_0_22)
        let s_0_23: i128 = u__id(state, tracer, s_0_22);
        // D s_0_24: cast reint s_0_23 -> i64
        let s_0_24: i64 = (s_0_23 as i64);
        // D s_0_25: cast zx s_0_2 -> i
        let s_0_25: i128 = (i128::try_from(s_0_2).unwrap());
        // D s_0_26: call __id(s_0_25)
        let s_0_26: i128 = u__id(state, tracer, s_0_25);
        // D s_0_27: cast reint s_0_26 -> i64
        let s_0_27: i64 = (s_0_26 as i64);
        // D s_0_28: cast zx s_0_24 -> i
        let s_0_28: i128 = (i128::try_from(s_0_24).unwrap());
        // D s_0_29: cast zx s_0_27 -> i
        let s_0_29: i128 = (i128::try_from(s_0_27).unwrap());
        // D s_0_30: add s_0_28 s_0_29
        let s_0_30: i128 = (s_0_28 + s_0_29);
        // D s_0_31: cast reint s_0_30 -> i64
        let s_0_31: i64 = (s_0_30 as i64);
        // C s_0_32: const #1s : i
        let s_0_32: i128 = 1;
        // D s_0_33: cast zx s_0_31 -> i
        let s_0_33: i128 = (i128::try_from(s_0_31).unwrap());
        // D s_0_34: sub s_0_33 s_0_32
        let s_0_34: i128 = ((s_0_33) - (s_0_32));
        // D s_0_35: cast reint s_0_34 -> i64
        let s_0_35: i64 = (s_0_34 as i64);
        // D s_0_36: cast zx s_0_2 -> i
        let s_0_36: i128 = (i128::try_from(s_0_2).unwrap());
        // D s_0_37: call __id(s_0_36)
        let s_0_37: i128 = u__id(state, tracer, s_0_36);
        // D s_0_38: cast reint s_0_37 -> i64
        let s_0_38: i64 = (s_0_37 as i64);
        // C s_0_39: const #2s : i
        let s_0_39: i128 = 2;
        // D s_0_40: cast zx s_0_38 -> i
        let s_0_40: i128 = (i128::try_from(s_0_38).unwrap());
        // D s_0_41: mul s_0_39 s_0_40
        let s_0_41: i128 = ((s_0_39) * (s_0_40));
        // D s_0_42: cast reint s_0_41 -> i64
        let s_0_42: i64 = (s_0_41 as i64);
        // D s_0_43: cast zx s_0_35 -> i
        let s_0_43: i128 = (i128::try_from(s_0_35).unwrap());
        // D s_0_44: cast zx s_0_42 -> i
        let s_0_44: i128 = (i128::try_from(s_0_42).unwrap());
        // D s_0_45: cmp-lt s_0_43 s_0_44
        let s_0_45: bool = ((s_0_43) < (s_0_44));
        // N s_0_46: assert s_0_45
        let s_0_46: () = assert!(s_0_45);
        // D s_0_47: cast zx s_0_2 -> i
        let s_0_47: i128 = (i128::try_from(s_0_2).unwrap());
        // D s_0_48: cast reint s_0_47 -> i64
        let s_0_48: i64 = (s_0_47 as i64);
        // D s_0_49: read-var lsb:i64
        let s_0_49: i64 = fn_state.lsb;
        // D s_0_50: cast zx s_0_49 -> i
        let s_0_50: i128 = (i128::try_from(s_0_49).unwrap());
        // D s_0_51: cast zx s_0_2 -> i
        let s_0_51: i128 = (i128::try_from(s_0_2).unwrap());
        // D s_0_52: add s_0_50 s_0_51
        let s_0_52: i128 = (s_0_50 + s_0_51);
        // D s_0_53: cast reint s_0_52 -> i64
        let s_0_53: i64 = (s_0_52 as i64);
        // C s_0_54: const #1s : i
        let s_0_54: i128 = 1;
        // D s_0_55: cast zx s_0_53 -> i
        let s_0_55: i128 = (i128::try_from(s_0_53).unwrap());
        // D s_0_56: sub s_0_55 s_0_54
        let s_0_56: i128 = ((s_0_55) - (s_0_54));
        // D s_0_57: cast reint s_0_56 -> i64
        let s_0_57: i64 = (s_0_56 as i64);
        // D s_0_58: cast zx s_0_57 -> i
        let s_0_58: i128 = (i128::try_from(s_0_57).unwrap());
        // D s_0_59: read-var lsb:i64
        let s_0_59: i64 = fn_state.lsb;
        // D s_0_60: cast zx s_0_59 -> i
        let s_0_60: i128 = (i128::try_from(s_0_59).unwrap());
        // C s_0_61: const #1s : i64
        let s_0_61: i64 = 1;
        // C s_0_62: cast zx s_0_61 -> i
        let s_0_62: i128 = (i128::try_from(s_0_61).unwrap());
        // D s_0_63: sub s_0_58 s_0_60
        let s_0_63: i128 = ((s_0_58) - (s_0_60));
        // D s_0_64: add s_0_63 s_0_62
        let s_0_64: i128 = (s_0_63 + s_0_62);
        // D s_0_65: bit-extract s_0_20 s_0_60 s_0_64
        let s_0_65: Bits = (Bits::new(
            ((s_0_20) >> (s_0_60)).value(),
            u16::try_from(s_0_64).unwrap(),
        ));
        // D s_0_66: read-var d:i64
        let s_0_66: i64 = fn_state.d;
        // D s_0_67: cast zx s_0_66 -> i
        let s_0_67: i128 = (i128::try_from(s_0_66).unwrap());
        // D s_0_68: call X_set(s_0_67, s_0_48, s_0_65)
        let s_0_68: () = X_set(state, tracer, s_0_67, s_0_48, s_0_65);
        // N s_0_69: return
        return;
    }
}
