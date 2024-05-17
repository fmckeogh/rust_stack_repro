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
pub fn execute_aarch32_instrs_CMN_rr_Op_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    m: i64,
    n: i64,
    s: i64,
    shift_t: u32,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_878707: ProductTyped54bc449dd09e5bd,
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
        // D s_0_27: cast zx s_0_26 -> bv
        let s_0_27: Bits = Bits::new(s_0_26 as u128, 32u16);
        // D s_0_28: cast zx s_0_23 -> bv
        let s_0_28: Bits = Bits::new(s_0_23 as u128, 32u16);
        // C s_0_29: const #0u : u8
        let s_0_29: bool = false;
        // D s_0_30: call AddWithCarry(s_0_27, s_0_28, s_0_29)
        let s_0_30: ProductTyped54bc449dd09e5bd = AddWithCarry(
            state,
            tracer,
            s_0_27,
            s_0_28,
            s_0_29,
        );
        // D s_0_31: write-var gs#878707 <= s_0_30
        fn_state.gs_878707 = s_0_30;
        // D s_0_32: read-var gs#878707.1:struct
        let s_0_32: u8 = fn_state.gs_878707._1;
        // C s_0_33: const #3s : i
        let s_0_33: i128 = 3;
        // D s_0_34: cast zx s_0_32 -> bv
        let s_0_34: Bits = Bits::new(s_0_32 as u128, 4u16);
        // C s_0_35: const #1s : i64
        let s_0_35: i64 = 1;
        // C s_0_36: cast zx s_0_35 -> i
        let s_0_36: i128 = (i128::try_from(s_0_35).unwrap());
        // C s_0_37: const #0s : i
        let s_0_37: i128 = 0;
        // C s_0_38: add s_0_37 s_0_36
        let s_0_38: i128 = (s_0_37 + s_0_36);
        // D s_0_39: bit-extract s_0_34 s_0_33 s_0_38
        let s_0_39: Bits = (Bits::new(
            ((s_0_34) >> (s_0_33)).value(),
            u16::try_from(s_0_38).unwrap(),
        ));
        // D s_0_40: cast reint s_0_39 -> u8
        let s_0_40: bool = ((s_0_39.value()) != 0);
        // C s_0_41: const #16984u : u32
        let s_0_41: u32 = 16984;
        // N s_0_42: write-reg s_0_41 <= s_0_40
        let s_0_42: () = {
            state.write_register::<bool>(s_0_41 as isize, s_0_40);
            tracer.write_register(s_0_41 as isize, s_0_40);
        };
        // C s_0_43: const #2s : i
        let s_0_43: i128 = 2;
        // D s_0_44: cast zx s_0_32 -> bv
        let s_0_44: Bits = Bits::new(s_0_32 as u128, 4u16);
        // C s_0_45: const #1s : i64
        let s_0_45: i64 = 1;
        // C s_0_46: cast zx s_0_45 -> i
        let s_0_46: i128 = (i128::try_from(s_0_45).unwrap());
        // C s_0_47: const #0s : i
        let s_0_47: i128 = 0;
        // C s_0_48: add s_0_47 s_0_46
        let s_0_48: i128 = (s_0_47 + s_0_46);
        // D s_0_49: bit-extract s_0_44 s_0_43 s_0_48
        let s_0_49: Bits = (Bits::new(
            ((s_0_44) >> (s_0_43)).value(),
            u16::try_from(s_0_48).unwrap(),
        ));
        // D s_0_50: cast reint s_0_49 -> u8
        let s_0_50: bool = ((s_0_49.value()) != 0);
        // C s_0_51: const #16997u : u32
        let s_0_51: u32 = 16997;
        // N s_0_52: write-reg s_0_51 <= s_0_50
        let s_0_52: () = {
            state.write_register::<bool>(s_0_51 as isize, s_0_50);
            tracer.write_register(s_0_51 as isize, s_0_50);
        };
        // C s_0_53: const #1s : i
        let s_0_53: i128 = 1;
        // D s_0_54: cast zx s_0_32 -> bv
        let s_0_54: Bits = Bits::new(s_0_32 as u128, 4u16);
        // C s_0_55: const #1s : i64
        let s_0_55: i64 = 1;
        // C s_0_56: cast zx s_0_55 -> i
        let s_0_56: i128 = (i128::try_from(s_0_55).unwrap());
        // C s_0_57: const #0s : i
        let s_0_57: i128 = 0;
        // C s_0_58: add s_0_57 s_0_56
        let s_0_58: i128 = (s_0_57 + s_0_56);
        // D s_0_59: bit-extract s_0_54 s_0_53 s_0_58
        let s_0_59: Bits = (Bits::new(
            ((s_0_54) >> (s_0_53)).value(),
            u16::try_from(s_0_58).unwrap(),
        ));
        // D s_0_60: cast reint s_0_59 -> u8
        let s_0_60: bool = ((s_0_59.value()) != 0);
        // C s_0_61: const #16971u : u32
        let s_0_61: u32 = 16971;
        // N s_0_62: write-reg s_0_61 <= s_0_60
        let s_0_62: () = {
            state.write_register::<bool>(s_0_61 as isize, s_0_60);
            tracer.write_register(s_0_61 as isize, s_0_60);
        };
        // C s_0_63: const #0s : i
        let s_0_63: i128 = 0;
        // D s_0_64: cast zx s_0_32 -> bv
        let s_0_64: Bits = Bits::new(s_0_32 as u128, 4u16);
        // C s_0_65: const #1s : i64
        let s_0_65: i64 = 1;
        // C s_0_66: cast zx s_0_65 -> i
        let s_0_66: i128 = (i128::try_from(s_0_65).unwrap());
        // C s_0_67: const #0s : i
        let s_0_67: i128 = 0;
        // C s_0_68: add s_0_67 s_0_66
        let s_0_68: i128 = (s_0_67 + s_0_66);
        // D s_0_69: bit-extract s_0_64 s_0_63 s_0_68
        let s_0_69: Bits = (Bits::new(
            ((s_0_64) >> (s_0_63)).value(),
            u16::try_from(s_0_68).unwrap(),
        ));
        // D s_0_70: cast reint s_0_69 -> u8
        let s_0_70: bool = ((s_0_69.value()) != 0);
        // C s_0_71: const #16996u : u32
        let s_0_71: u32 = 16996;
        // N s_0_72: write-reg s_0_71 <= s_0_70
        let s_0_72: () = {
            state.write_register::<bool>(s_0_71 as isize, s_0_70);
            tracer.write_register(s_0_71 as isize, s_0_70);
        };
        // N s_0_73: return
        return;
    }
}
