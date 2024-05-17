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
use Bit::*;
use common::*;
pub fn DBGDSCRint_write<T: Tracer>(
    state: &mut State,
    tracer: &T,
    value_name: ProductType700c18a878c5601b,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        r: ProductType700c18a878c5601b,
        value_name: ProductType700c18a878c5601b,
    }
    let fn_state = FunctionState {
        value_name,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_0_0: read-var value_name:struct
        let s_0_0: ProductType700c18a878c5601b = fn_state.value_name;
        // D s_0_1: write-var r <= s_0_0
        fn_state.r = s_0_0;
        // C s_0_2: const #16832u : u32
        let s_0_2: u32 = 16832;
        // D s_0_3: read-reg s_0_2:struct
        let s_0_3: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_2 as isize);
            tracer.read_register(s_0_2 as isize, value);
            value
        };
        // C s_0_4: const #16832u : u32
        let s_0_4: u32 = 16832;
        // N s_0_5: write-reg s_0_4 <= s_0_3
        let s_0_5: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_0_4 as isize, s_0_3);
            tracer.write_register(s_0_4 as isize, s_0_3);
        };
        // D s_0_6: read-var r.0:struct
        let s_0_6: u32 = fn_state.r._0;
        // C s_0_7: const #15s : i
        let s_0_7: i128 = 15;
        // C s_0_8: const #1s : i
        let s_0_8: i128 = 1;
        // D s_0_9: cast zx s_0_6 -> bv
        let s_0_9: Bits = Bits::new(s_0_6 as u128, 32u16);
        // D s_0_10: bit-extract s_0_9 s_0_7 s_0_8
        let s_0_10: Bits = (Bits::new(
            ((s_0_9) >> (s_0_7)).value(),
            u16::try_from(s_0_8).unwrap(),
        ));
        // D s_0_11: cast reint s_0_10 -> u8
        let s_0_11: bool = ((s_0_10.value()) != 0);
        // D s_0_12: call Bit(s_0_11)
        let s_0_12: bool = Bit(state, tracer, s_0_11);
        // C s_0_13: const #104648u : u32
        let s_0_13: u32 = 104648;
        // D s_0_14: read-reg s_0_13:struct
        let s_0_14: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_13 as isize);
            tracer.read_register(s_0_13 as isize, value);
            value
        };
        // C s_0_15: const #104648u : u32
        let s_0_15: u32 = 104648;
        // N s_0_16: write-reg s_0_15 <= s_0_14
        let s_0_16: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_0_15 as isize, s_0_14);
            tracer.write_register(s_0_15 as isize, s_0_14);
        };
        // D s_0_17: read-var r.0:struct
        let s_0_17: u32 = fn_state.r._0;
        // C s_0_18: const #12s : i
        let s_0_18: i128 = 12;
        // C s_0_19: const #1s : i
        let s_0_19: i128 = 1;
        // D s_0_20: cast zx s_0_17 -> bv
        let s_0_20: Bits = Bits::new(s_0_17 as u128, 32u16);
        // D s_0_21: bit-extract s_0_20 s_0_18 s_0_19
        let s_0_21: Bits = (Bits::new(
            ((s_0_20) >> (s_0_18)).value(),
            u16::try_from(s_0_19).unwrap(),
        ));
        // D s_0_22: cast reint s_0_21 -> u8
        let s_0_22: bool = ((s_0_21.value()) != 0);
        // D s_0_23: call Bit(s_0_22)
        let s_0_23: bool = Bit(state, tracer, s_0_22);
        // C s_0_24: const #104648u : u32
        let s_0_24: u32 = 104648;
        // D s_0_25: read-reg s_0_24:struct
        let s_0_25: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_24 as isize);
            tracer.read_register(s_0_24 as isize, value);
            value
        };
        // C s_0_26: const #104648u : u32
        let s_0_26: u32 = 104648;
        // N s_0_27: write-reg s_0_26 <= s_0_25
        let s_0_27: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_0_26 as isize, s_0_25);
            tracer.write_register(s_0_26 as isize, s_0_25);
        };
        // C s_0_28: const #104648u : u32
        let s_0_28: u32 = 104648;
        // D s_0_29: read-reg s_0_28:struct
        let s_0_29: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_28 as isize);
            tracer.read_register(s_0_28 as isize, value);
            value
        };
        // C s_0_30: const #104648u : u32
        let s_0_30: u32 = 104648;
        // N s_0_31: write-reg s_0_30 <= s_0_29
        let s_0_31: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_0_30 as isize, s_0_29);
            tracer.write_register(s_0_30 as isize, s_0_29);
        };
        // C s_0_32: const #23040u : u32
        let s_0_32: u32 = 23040;
        // N s_0_33: write-reg s_0_32 <= s_0_0
        let s_0_33: () = {
            state.write_register::<ProductType700c18a878c5601b>(s_0_32 as isize, s_0_0);
            tracer.write_register(s_0_32 as isize, s_0_0);
        };
        // N s_0_34: return
        return;
    }
}
