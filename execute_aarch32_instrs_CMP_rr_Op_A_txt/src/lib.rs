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
use Shift::*;
use AddWithCarry::*;
use R_read::*;
use common::*;
pub fn execute_aarch32_instrs_CMP_rr_Op_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    m: i64,
    n: i64,
    s: i64,
    shift_t: u32,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_878817: ProductTyped54bc449dd09e5bd,
        m: i64,
        n: i64,
        s: i64,
        shift_t: u32,
    }
    let fn_state = FunctionState {
        m,
        n,
        s,
        shift_t,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_0_0: read-var s:i64
        let s_0_0: i64 = fn_state.s;
        // D s_0_1: cast zx s_0_0 -> i
        let s_0_1: i128 = (i128::try_from(s_0_0).unwrap());
        // D s_0_2: call R_read(s_0_1)
        let s_0_2: u32 = R_read(state, tracer, s_0_1);
        // C s_0_3: const #0s : i
        let s_0_3: i128 = 0;
        // D s_0_4: cast zx s_0_2 -> bv
        let s_0_4: Bits = Bits::new(s_0_2 as u128, 32u16);
        // C s_0_5: const #1s : i64
        let s_0_5: i64 = 1;
        // C s_0_6: cast zx s_0_5 -> i
        let s_0_6: i128 = (i128::try_from(s_0_5).unwrap());
        // C s_0_7: const #7s : i
        let s_0_7: i128 = 7;
        // C s_0_8: add s_0_7 s_0_6
        let s_0_8: i128 = (s_0_7 + s_0_6);
        // D s_0_9: bit-extract s_0_4 s_0_3 s_0_8
        let s_0_9: Bits = (Bits::new(
            ((s_0_4) >> (s_0_3)).value(),
            u16::try_from(s_0_8).unwrap(),
        ));
        // D s_0_10: cast reint s_0_9 -> u8
        let s_0_10: u8 = (s_0_9.value() as u8);
        // D s_0_11: cast zx s_0_10 -> bv
        let s_0_11: Bits = Bits::new(s_0_10 as u128, 8u16);
        // D s_0_12: cast zx s_0_11 -> i
        let s_0_12: i128 = (s_0_11.value() as i128);
        // D s_0_13: cast reint s_0_12 -> i64
        let s_0_13: i64 = (s_0_12 as i64);
        // D s_0_14: read-var m:i64
        let s_0_14: i64 = fn_state.m;
        // D s_0_15: cast zx s_0_14 -> i
        let s_0_15: i128 = (i128::try_from(s_0_14).unwrap());
        // D s_0_16: call R_read(s_0_15)
        let s_0_16: u32 = R_read(state, tracer, s_0_15);
        // C s_0_17: const #16971u : u32
        let s_0_17: u32 = 16971;
        // D s_0_18: read-reg s_0_17:u8
        let s_0_18: bool = {
            let value = state.read_register::<bool>(s_0_17 as isize);
            tracer.read_register(s_0_17 as isize, value);
            value
        };
        // D s_0_19: cast zx s_0_16 -> bv
        let s_0_19: Bits = Bits::new(s_0_16 as u128, 32u16);
        // D s_0_20: cast zx s_0_13 -> i
        let s_0_20: i128 = (i128::try_from(s_0_13).unwrap());
        // D s_0_21: read-var shift_t:u32
        let s_0_21: u32 = fn_state.shift_t;
        // D s_0_22: call Shift(s_0_19, s_0_21, s_0_20, s_0_18)
        let s_0_22: Bits = Shift(state, tracer, s_0_19, s_0_21, s_0_20, s_0_18);
        // D s_0_23: cast reint s_0_22 -> u32
        let s_0_23: u32 = (s_0_22.value() as u32);
        // D s_0_24: read-var n:i64
        let s_0_24: i64 = fn_state.n;
        // D s_0_25: cast zx s_0_24 -> i
        let s_0_25: i128 = (i128::try_from(s_0_24).unwrap());
        // D s_0_26: call R_read(s_0_25)
        let s_0_26: u32 = R_read(state, tracer, s_0_25);
        // D s_0_27: cast zx s_0_23 -> bv
        let s_0_27: Bits = Bits::new(s_0_23 as u128, 32u16);
        // D s_0_28: not s_0_27
        let s_0_28: Bits = !s_0_27;
        // D s_0_29: cast reint s_0_28 -> u32
        let s_0_29: u32 = (s_0_28.value() as u32);
        // D s_0_30: cast zx s_0_26 -> bv
        let s_0_30: Bits = Bits::new(s_0_26 as u128, 32u16);
        // D s_0_31: cast zx s_0_29 -> bv
        let s_0_31: Bits = Bits::new(s_0_29 as u128, 32u16);
        // C s_0_32: const #1u : u8
        let s_0_32: bool = true;
        // D s_0_33: call AddWithCarry(s_0_30, s_0_31, s_0_32)
        let s_0_33: ProductTyped54bc449dd09e5bd = AddWithCarry(
            state,
            tracer,
            s_0_30,
            s_0_31,
            s_0_32,
        );
        // D s_0_34: write-var gs#878817 <= s_0_33
        fn_state.gs_878817 = s_0_33;
        // D s_0_35: read-var gs#878817.1:struct
        let s_0_35: u8 = fn_state.gs_878817._1;
        // C s_0_36: const #3s : i
        let s_0_36: i128 = 3;
        // D s_0_37: cast zx s_0_35 -> bv
        let s_0_37: Bits = Bits::new(s_0_35 as u128, 4u16);
        // C s_0_38: const #1s : i64
        let s_0_38: i64 = 1;
        // C s_0_39: cast zx s_0_38 -> i
        let s_0_39: i128 = (i128::try_from(s_0_38).unwrap());
        // C s_0_40: const #0s : i
        let s_0_40: i128 = 0;
        // C s_0_41: add s_0_40 s_0_39
        let s_0_41: i128 = (s_0_40 + s_0_39);
        // D s_0_42: bit-extract s_0_37 s_0_36 s_0_41
        let s_0_42: Bits = (Bits::new(
            ((s_0_37) >> (s_0_36)).value(),
            u16::try_from(s_0_41).unwrap(),
        ));
        // D s_0_43: cast reint s_0_42 -> u8
        let s_0_43: bool = ((s_0_42.value()) != 0);
        // C s_0_44: const #16984u : u32
        let s_0_44: u32 = 16984;
        // N s_0_45: write-reg s_0_44 <= s_0_43
        let s_0_45: () = {
            state.write_register::<bool>(s_0_44 as isize, s_0_43);
            tracer.write_register(s_0_44 as isize, s_0_43);
        };
        // C s_0_46: const #2s : i
        let s_0_46: i128 = 2;
        // D s_0_47: cast zx s_0_35 -> bv
        let s_0_47: Bits = Bits::new(s_0_35 as u128, 4u16);
        // C s_0_48: const #1s : i64
        let s_0_48: i64 = 1;
        // C s_0_49: cast zx s_0_48 -> i
        let s_0_49: i128 = (i128::try_from(s_0_48).unwrap());
        // C s_0_50: const #0s : i
        let s_0_50: i128 = 0;
        // C s_0_51: add s_0_50 s_0_49
        let s_0_51: i128 = (s_0_50 + s_0_49);
        // D s_0_52: bit-extract s_0_47 s_0_46 s_0_51
        let s_0_52: Bits = (Bits::new(
            ((s_0_47) >> (s_0_46)).value(),
            u16::try_from(s_0_51).unwrap(),
        ));
        // D s_0_53: cast reint s_0_52 -> u8
        let s_0_53: bool = ((s_0_52.value()) != 0);
        // C s_0_54: const #16997u : u32
        let s_0_54: u32 = 16997;
        // N s_0_55: write-reg s_0_54 <= s_0_53
        let s_0_55: () = {
            state.write_register::<bool>(s_0_54 as isize, s_0_53);
            tracer.write_register(s_0_54 as isize, s_0_53);
        };
        // C s_0_56: const #1s : i
        let s_0_56: i128 = 1;
        // D s_0_57: cast zx s_0_35 -> bv
        let s_0_57: Bits = Bits::new(s_0_35 as u128, 4u16);
        // C s_0_58: const #1s : i64
        let s_0_58: i64 = 1;
        // C s_0_59: cast zx s_0_58 -> i
        let s_0_59: i128 = (i128::try_from(s_0_58).unwrap());
        // C s_0_60: const #0s : i
        let s_0_60: i128 = 0;
        // C s_0_61: add s_0_60 s_0_59
        let s_0_61: i128 = (s_0_60 + s_0_59);
        // D s_0_62: bit-extract s_0_57 s_0_56 s_0_61
        let s_0_62: Bits = (Bits::new(
            ((s_0_57) >> (s_0_56)).value(),
            u16::try_from(s_0_61).unwrap(),
        ));
        // D s_0_63: cast reint s_0_62 -> u8
        let s_0_63: bool = ((s_0_62.value()) != 0);
        // C s_0_64: const #16971u : u32
        let s_0_64: u32 = 16971;
        // N s_0_65: write-reg s_0_64 <= s_0_63
        let s_0_65: () = {
            state.write_register::<bool>(s_0_64 as isize, s_0_63);
            tracer.write_register(s_0_64 as isize, s_0_63);
        };
        // C s_0_66: const #0s : i
        let s_0_66: i128 = 0;
        // D s_0_67: cast zx s_0_35 -> bv
        let s_0_67: Bits = Bits::new(s_0_35 as u128, 4u16);
        // C s_0_68: const #1s : i64
        let s_0_68: i64 = 1;
        // C s_0_69: cast zx s_0_68 -> i
        let s_0_69: i128 = (i128::try_from(s_0_68).unwrap());
        // C s_0_70: const #0s : i
        let s_0_70: i128 = 0;
        // C s_0_71: add s_0_70 s_0_69
        let s_0_71: i128 = (s_0_70 + s_0_69);
        // D s_0_72: bit-extract s_0_67 s_0_66 s_0_71
        let s_0_72: Bits = (Bits::new(
            ((s_0_67) >> (s_0_66)).value(),
            u16::try_from(s_0_71).unwrap(),
        ));
        // D s_0_73: cast reint s_0_72 -> u8
        let s_0_73: bool = ((s_0_72.value()) != 0);
        // C s_0_74: const #16996u : u32
        let s_0_74: u32 = 16996;
        // N s_0_75: write-reg s_0_74 <= s_0_73
        let s_0_75: () = {
            state.write_register::<bool>(s_0_74 as isize, s_0_73);
            tracer.write_register(s_0_74 as isize, s_0_73);
        };
        // N s_0_76: return
        return;
    }
}
