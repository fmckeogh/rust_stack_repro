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
use AddWithCarry::*;
use R_read::*;
use common::*;
pub fn execute_aarch32_instrs_CMP_i_Op_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    imm32: u32,
    n: i64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_878730: ProductTyped54bc449dd09e5bd,
        imm32: u32,
        n: i64,
    }
    let fn_state = FunctionState {
        imm32,
        n,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_0_0: read-var n:i64
        let s_0_0: i64 = fn_state.n;
        // D s_0_1: cast zx s_0_0 -> i
        let s_0_1: i128 = (i128::try_from(s_0_0).unwrap());
        // D s_0_2: call R_read(s_0_1)
        let s_0_2: u32 = R_read(state, tracer, s_0_1);
        // D s_0_3: read-var imm32:u32
        let s_0_3: u32 = fn_state.imm32;
        // D s_0_4: cast zx s_0_3 -> bv
        let s_0_4: Bits = Bits::new(s_0_3 as u128, 32u16);
        // D s_0_5: not s_0_4
        let s_0_5: Bits = !s_0_4;
        // D s_0_6: cast reint s_0_5 -> u32
        let s_0_6: u32 = (s_0_5.value() as u32);
        // D s_0_7: cast zx s_0_2 -> bv
        let s_0_7: Bits = Bits::new(s_0_2 as u128, 32u16);
        // D s_0_8: cast zx s_0_6 -> bv
        let s_0_8: Bits = Bits::new(s_0_6 as u128, 32u16);
        // C s_0_9: const #1u : u8
        let s_0_9: bool = true;
        // D s_0_10: call AddWithCarry(s_0_7, s_0_8, s_0_9)
        let s_0_10: ProductTyped54bc449dd09e5bd = AddWithCarry(
            state,
            tracer,
            s_0_7,
            s_0_8,
            s_0_9,
        );
        // D s_0_11: write-var gs#878730 <= s_0_10
        fn_state.gs_878730 = s_0_10;
        // D s_0_12: read-var gs#878730.1:struct
        let s_0_12: u8 = fn_state.gs_878730._1;
        // C s_0_13: const #3s : i
        let s_0_13: i128 = 3;
        // D s_0_14: cast zx s_0_12 -> bv
        let s_0_14: Bits = Bits::new(s_0_12 as u128, 4u16);
        // C s_0_15: const #1s : i64
        let s_0_15: i64 = 1;
        // C s_0_16: cast zx s_0_15 -> i
        let s_0_16: i128 = (i128::try_from(s_0_15).unwrap());
        // C s_0_17: const #0s : i
        let s_0_17: i128 = 0;
        // C s_0_18: add s_0_17 s_0_16
        let s_0_18: i128 = (s_0_17 + s_0_16);
        // D s_0_19: bit-extract s_0_14 s_0_13 s_0_18
        let s_0_19: Bits = (Bits::new(
            ((s_0_14) >> (s_0_13)).value(),
            u16::try_from(s_0_18).unwrap(),
        ));
        // D s_0_20: cast reint s_0_19 -> u8
        let s_0_20: bool = ((s_0_19.value()) != 0);
        // C s_0_21: const #16984u : u32
        let s_0_21: u32 = 16984;
        // N s_0_22: write-reg s_0_21 <= s_0_20
        let s_0_22: () = {
            state.write_register::<bool>(s_0_21 as isize, s_0_20);
            tracer.write_register(s_0_21 as isize, s_0_20);
        };
        // C s_0_23: const #2s : i
        let s_0_23: i128 = 2;
        // D s_0_24: cast zx s_0_12 -> bv
        let s_0_24: Bits = Bits::new(s_0_12 as u128, 4u16);
        // C s_0_25: const #1s : i64
        let s_0_25: i64 = 1;
        // C s_0_26: cast zx s_0_25 -> i
        let s_0_26: i128 = (i128::try_from(s_0_25).unwrap());
        // C s_0_27: const #0s : i
        let s_0_27: i128 = 0;
        // C s_0_28: add s_0_27 s_0_26
        let s_0_28: i128 = (s_0_27 + s_0_26);
        // D s_0_29: bit-extract s_0_24 s_0_23 s_0_28
        let s_0_29: Bits = (Bits::new(
            ((s_0_24) >> (s_0_23)).value(),
            u16::try_from(s_0_28).unwrap(),
        ));
        // D s_0_30: cast reint s_0_29 -> u8
        let s_0_30: bool = ((s_0_29.value()) != 0);
        // C s_0_31: const #16997u : u32
        let s_0_31: u32 = 16997;
        // N s_0_32: write-reg s_0_31 <= s_0_30
        let s_0_32: () = {
            state.write_register::<bool>(s_0_31 as isize, s_0_30);
            tracer.write_register(s_0_31 as isize, s_0_30);
        };
        // C s_0_33: const #1s : i
        let s_0_33: i128 = 1;
        // D s_0_34: cast zx s_0_12 -> bv
        let s_0_34: Bits = Bits::new(s_0_12 as u128, 4u16);
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
        // C s_0_41: const #16971u : u32
        let s_0_41: u32 = 16971;
        // N s_0_42: write-reg s_0_41 <= s_0_40
        let s_0_42: () = {
            state.write_register::<bool>(s_0_41 as isize, s_0_40);
            tracer.write_register(s_0_41 as isize, s_0_40);
        };
        // C s_0_43: const #0s : i
        let s_0_43: i128 = 0;
        // D s_0_44: cast zx s_0_12 -> bv
        let s_0_44: Bits = Bits::new(s_0_12 as u128, 4u16);
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
        // C s_0_51: const #16996u : u32
        let s_0_51: u32 = 16996;
        // N s_0_52: write-reg s_0_51 <= s_0_50
        let s_0_52: () = {
            state.write_register::<bool>(s_0_51 as isize, s_0_50);
            tracer.write_register(s_0_51 as isize, s_0_50);
        };
        // N s_0_53: return
        return;
    }
}
