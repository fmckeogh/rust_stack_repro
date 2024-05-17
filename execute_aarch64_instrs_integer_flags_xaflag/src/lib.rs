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
use common::*;
pub fn execute_aarch64_instrs_integer_flags_xaflag<T: Tracer>(
    state: &mut State,
    tracer: &T,
    gs_174365: (),
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_174365: (),
    }
    let fn_state = FunctionState {
        gs_174365,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_0_0: const #16971u : u32
        let s_0_0: u32 = 16971;
        // D s_0_1: read-reg s_0_0:u8
        let s_0_1: bool = {
            let value = state.read_register::<bool>(s_0_0 as isize);
            tracer.read_register(s_0_0 as isize, value);
            value
        };
        // D s_0_2: cast zx s_0_1 -> bv
        let s_0_2: Bits = Bits::new(s_0_1 as u128, 1u16);
        // D s_0_3: not s_0_2
        let s_0_3: Bits = !s_0_2;
        // D s_0_4: cast reint s_0_3 -> u8
        let s_0_4: bool = ((s_0_3.value()) != 0);
        // C s_0_5: const #16997u : u32
        let s_0_5: u32 = 16997;
        // D s_0_6: read-reg s_0_5:u8
        let s_0_6: bool = {
            let value = state.read_register::<bool>(s_0_5 as isize);
            tracer.read_register(s_0_5 as isize, value);
            value
        };
        // D s_0_7: cast zx s_0_6 -> bv
        let s_0_7: Bits = Bits::new(s_0_6 as u128, 1u16);
        // D s_0_8: not s_0_7
        let s_0_8: Bits = !s_0_7;
        // D s_0_9: cast reint s_0_8 -> u8
        let s_0_9: bool = ((s_0_8.value()) != 0);
        // D s_0_10: cast zx s_0_4 -> bv
        let s_0_10: Bits = Bits::new(s_0_4 as u128, 1u16);
        // D s_0_11: cast zx s_0_9 -> bv
        let s_0_11: Bits = Bits::new(s_0_9 as u128, 1u16);
        // D s_0_12: and s_0_10 s_0_11
        let s_0_12: Bits = ((s_0_10) & (s_0_11));
        // D s_0_13: cast reint s_0_12 -> u8
        let s_0_13: bool = ((s_0_12.value()) != 0);
        // C s_0_14: const #16997u : u32
        let s_0_14: u32 = 16997;
        // D s_0_15: read-reg s_0_14:u8
        let s_0_15: bool = {
            let value = state.read_register::<bool>(s_0_14 as isize);
            tracer.read_register(s_0_14 as isize, value);
            value
        };
        // C s_0_16: const #16971u : u32
        let s_0_16: u32 = 16971;
        // D s_0_17: read-reg s_0_16:u8
        let s_0_17: bool = {
            let value = state.read_register::<bool>(s_0_16 as isize);
            tracer.read_register(s_0_16 as isize, value);
            value
        };
        // D s_0_18: cast zx s_0_15 -> bv
        let s_0_18: Bits = Bits::new(s_0_15 as u128, 1u16);
        // D s_0_19: cast zx s_0_17 -> bv
        let s_0_19: Bits = Bits::new(s_0_17 as u128, 1u16);
        // D s_0_20: and s_0_18 s_0_19
        let s_0_20: Bits = ((s_0_18) & (s_0_19));
        // D s_0_21: cast reint s_0_20 -> u8
        let s_0_21: bool = ((s_0_20.value()) != 0);
        // C s_0_22: const #16971u : u32
        let s_0_22: u32 = 16971;
        // D s_0_23: read-reg s_0_22:u8
        let s_0_23: bool = {
            let value = state.read_register::<bool>(s_0_22 as isize);
            tracer.read_register(s_0_22 as isize, value);
            value
        };
        // C s_0_24: const #16997u : u32
        let s_0_24: u32 = 16997;
        // D s_0_25: read-reg s_0_24:u8
        let s_0_25: bool = {
            let value = state.read_register::<bool>(s_0_24 as isize);
            tracer.read_register(s_0_24 as isize, value);
            value
        };
        // D s_0_26: cast zx s_0_23 -> bv
        let s_0_26: Bits = Bits::new(s_0_23 as u128, 1u16);
        // D s_0_27: cast zx s_0_25 -> bv
        let s_0_27: Bits = Bits::new(s_0_25 as u128, 1u16);
        // D s_0_28: or s_0_26 s_0_27
        let s_0_28: Bits = ((s_0_26) | (s_0_27));
        // D s_0_29: cast reint s_0_28 -> u8
        let s_0_29: bool = ((s_0_28.value()) != 0);
        // C s_0_30: const #16971u : u32
        let s_0_30: u32 = 16971;
        // D s_0_31: read-reg s_0_30:u8
        let s_0_31: bool = {
            let value = state.read_register::<bool>(s_0_30 as isize);
            tracer.read_register(s_0_30 as isize, value);
            value
        };
        // D s_0_32: cast zx s_0_31 -> bv
        let s_0_32: Bits = Bits::new(s_0_31 as u128, 1u16);
        // D s_0_33: not s_0_32
        let s_0_33: Bits = !s_0_32;
        // D s_0_34: cast reint s_0_33 -> u8
        let s_0_34: bool = ((s_0_33.value()) != 0);
        // C s_0_35: const #16997u : u32
        let s_0_35: u32 = 16997;
        // D s_0_36: read-reg s_0_35:u8
        let s_0_36: bool = {
            let value = state.read_register::<bool>(s_0_35 as isize);
            tracer.read_register(s_0_35 as isize, value);
            value
        };
        // D s_0_37: cast zx s_0_34 -> bv
        let s_0_37: Bits = Bits::new(s_0_34 as u128, 1u16);
        // D s_0_38: cast zx s_0_36 -> bv
        let s_0_38: Bits = Bits::new(s_0_36 as u128, 1u16);
        // D s_0_39: and s_0_37 s_0_38
        let s_0_39: Bits = ((s_0_37) & (s_0_38));
        // D s_0_40: cast reint s_0_39 -> u8
        let s_0_40: bool = ((s_0_39.value()) != 0);
        // C s_0_41: const #16984u : u32
        let s_0_41: u32 = 16984;
        // N s_0_42: write-reg s_0_41 <= s_0_13
        let s_0_42: () = {
            state.write_register::<bool>(s_0_41 as isize, s_0_13);
            tracer.write_register(s_0_41 as isize, s_0_13);
        };
        // C s_0_43: const #16997u : u32
        let s_0_43: u32 = 16997;
        // N s_0_44: write-reg s_0_43 <= s_0_21
        let s_0_44: () = {
            state.write_register::<bool>(s_0_43 as isize, s_0_21);
            tracer.write_register(s_0_43 as isize, s_0_21);
        };
        // C s_0_45: const #16971u : u32
        let s_0_45: u32 = 16971;
        // N s_0_46: write-reg s_0_45 <= s_0_29
        let s_0_46: () = {
            state.write_register::<bool>(s_0_45 as isize, s_0_29);
            tracer.write_register(s_0_45 as isize, s_0_29);
        };
        // C s_0_47: const #16996u : u32
        let s_0_47: u32 = 16996;
        // N s_0_48: write-reg s_0_47 <= s_0_40
        let s_0_48: () = {
            state.write_register::<bool>(s_0_47 as isize, s_0_40);
            tracer.write_register(s_0_47 as isize, s_0_40);
        };
        // N s_0_49: return
        return;
    }
}
