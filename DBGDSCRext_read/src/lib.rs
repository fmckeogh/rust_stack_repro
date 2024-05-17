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
use DBGDSCRint_read::*;
use Bit::*;
use common::*;
pub fn DBGDSCRext_read<T: Tracer>(
    state: &mut State,
    tracer: &T,
    gs_9409: (),
) -> ProductType700c18a878c5601b {
    #[derive(Default)]
    struct FunctionState {
        ga_6562: ProductType700c18a878c5601b,
        ga_6568: ProductType700c18a878c5601b,
        gs_9409: (),
    }
    let fn_state = FunctionState {
        gs_9409,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType700c18a878c5601b {
        // C s_0_0: const #17648u : u32
        let s_0_0: u32 = 17648;
        // D s_0_1: read-reg s_0_0:struct
        let s_0_1: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_0_0 as isize);
            tracer.read_register(s_0_0 as isize, value);
            value
        };
        // C s_0_2: const #() : ()
        let s_0_2: () = ();
        // S s_0_3: call DBGDSCRint_read(s_0_2)
        let s_0_3: ProductType700c18a878c5601b = DBGDSCRint_read(state, tracer, s_0_2);
        // D s_0_4: write-var ga#6562 <= s_0_3
        fn_state.ga_6562 = s_0_3;
        // D s_0_5: read-var ga#6562.0:struct
        let s_0_5: u32 = fn_state.ga_6562._0;
        // C s_0_6: const #15s : i
        let s_0_6: i128 = 15;
        // C s_0_7: const #1s : i
        let s_0_7: i128 = 1;
        // D s_0_8: cast zx s_0_5 -> bv
        let s_0_8: Bits = Bits::new(s_0_5 as u128, 32u16);
        // D s_0_9: bit-extract s_0_8 s_0_6 s_0_7
        let s_0_9: Bits = (Bits::new(
            ((s_0_8) >> (s_0_6)).value(),
            u16::try_from(s_0_7).unwrap(),
        ));
        // D s_0_10: cast reint s_0_9 -> u8
        let s_0_10: bool = ((s_0_9.value()) != 0);
        // D s_0_11: call Bit(s_0_10)
        let s_0_11: bool = Bit(state, tracer, s_0_10);
        // C s_0_12: const #() : ()
        let s_0_12: () = ();
        // S s_0_13: call DBGDSCRint_read(s_0_12)
        let s_0_13: ProductType700c18a878c5601b = DBGDSCRint_read(state, tracer, s_0_12);
        // D s_0_14: write-var ga#6568 <= s_0_13
        fn_state.ga_6568 = s_0_13;
        // D s_0_15: read-var ga#6568.0:struct
        let s_0_15: u32 = fn_state.ga_6568._0;
        // C s_0_16: const #12s : i
        let s_0_16: i128 = 12;
        // C s_0_17: const #1s : i
        let s_0_17: i128 = 1;
        // D s_0_18: cast zx s_0_15 -> bv
        let s_0_18: Bits = Bits::new(s_0_15 as u128, 32u16);
        // D s_0_19: bit-extract s_0_18 s_0_16 s_0_17
        let s_0_19: Bits = (Bits::new(
            ((s_0_18) >> (s_0_16)).value(),
            u16::try_from(s_0_17).unwrap(),
        ));
        // D s_0_20: cast reint s_0_19 -> u8
        let s_0_20: bool = ((s_0_19.value()) != 0);
        // D s_0_21: call Bit(s_0_20)
        let s_0_21: bool = Bit(state, tracer, s_0_20);
        // C s_0_22: const #() : ()
        let s_0_22: () = ();
        // S s_0_23: call DBGDSCRint_read(s_0_22)
        let s_0_23: ProductType700c18a878c5601b = DBGDSCRint_read(state, tracer, s_0_22);
        // N s_0_24: return s_0_1
        return s_0_1;
    }
}
